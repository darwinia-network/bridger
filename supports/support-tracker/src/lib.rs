use itertools::Itertools;
use microkv::namespace::NamespaceMicroKV;

#[derive(Clone)]
pub struct Tracker {
    microkv: NamespaceMicroKV,
    key_raw: String,
    key_curt: String,
    key_state: String, // todo: change use key_running and type is bool
    key_next: String,
    key_skipped: String,
}

impl Tracker {
    pub fn new(microkv: NamespaceMicroKV, key: impl AsRef<str>) -> Self {
        let key = key.as_ref();
        Self {
            microkv,
            key_raw: key.to_string(),
            key_curt: format!("{}.current", key),
            key_state: format!("{}.state", key),
            key_next: format!("{}.next", key),
            key_skipped: format!("{}.skipped", key),
        }
    }
}

impl Tracker {
    pub fn state(&self) -> anyhow::Result<TrackState> {
        let v = self
            .microkv
            .get_as(&self.key_state)?
            .map(|v: String| v.to_lowercase());
        if Some("running".to_string()) == v {
            return Ok(TrackState::Running);
        }
        Ok(TrackState::Paused)
    }

    pub async fn next(&self) -> anyhow::Result<usize> {
        let state = self.state()?;
        if state == TrackState::Paused {
            loop {
                let secs = 3;
                log::warn!(
                    "The track key [{}] state is {:?}, wait {} seconds check again.",
                    &self.key_state,
                    state,
                    secs
                );
                tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
                let state = self.state()?;
                if state != TrackState::Paused {
                    break;
                }
            }
        }
        let next: Option<String> = self.microkv.get_as(&self.key_next)?;
        if next.is_none() {
            let curt: usize = self.microkv.get_as(&self.key_curt)?.unwrap_or(0);
            let next = curt + 1;
            self.microkv.put(&self.key_curt, &next)?;
            return Ok(next);
        }
        let mut plan_blocks = parse_blocks_from_text(next.unwrap())?;
        let next = plan_blocks.first().unwrap_or(&1).clone();
        if !plan_blocks.is_empty() {
            plan_blocks.remove(0);
            let store_value: String = plan_blocks.iter().join(",");
            if !plan_blocks.is_empty() {
                self.microkv.put(&self.key_next, &store_value)?;
            }
        }
        if plan_blocks.is_empty() {
            self.microkv.delete(&self.key_next)?;
        }
        self.microkv.put(&self.key_curt, &next)?;
        Ok(next)
    }

    pub fn skip(&self, block: usize) -> anyhow::Result<()> {
        let skipped: Option<String> = self.microkv.get_as(&self.key_skipped)?;
        match skipped {
            Some(v) => self
                .microkv
                .put(&self.key_skipped, &format!("{},{}", v, block))?,
            None => self.microkv.put(&self.key_skipped, &format!("{}", block))?,
        }
        Ok(())
    }

    pub fn retry_skipped(&self) -> anyhow::Result<()> {
        let skipped: Option<String> = self.microkv.get_as(&self.key_skipped)?;
        if let Some(v) = skipped {
            let blocks = parse_blocks_from_text(v)?;
            self.jump_the_queue(blocks)?;
            self.microkv.delete(&self.key_skipped)?;
        }
        Ok(())
    }

    pub fn jump_the_queue(&self, mut blocks: Vec<usize>) -> anyhow::Result<()> {
        let curt: Option<usize> = self.microkv.get_as(&self.key_curt)?;
        if let Some(v) = curt {
            blocks.push(v);
        }
        let value: String = blocks.iter().join(",");
        self.microkv.put(&self.key_next, &value)?;
        Ok(())
    }
}

fn parse_blocks_from_text(text: String) -> anyhow::Result<Vec<usize>> {
    let text = text.trim();
    let mut blocks = vec![];
    if text.starts_with("[") && text.ends_with("]") {
        blocks = serde_json::from_str(text)?;
    } else {
        let arrs = text.split(",").collect::<Vec<&str>>();
        for item in arrs {
            let item = item.trim();
            match item.parse::<usize>() {
                Ok(v) => blocks.push(v),
                Err(_) => {}
            }
        }
    }
    Ok(blocks)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TrackState {
    Running,
    Paused,
}
