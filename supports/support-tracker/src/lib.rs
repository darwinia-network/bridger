use itertools::Itertools;
use microkv::namespace::NamespaceMicroKV;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Tracker {
    microkv: NamespaceMicroKV,
    key_raw: String,
    key_curt: String,
    key_finish: String,
    key_running: String,
    key_next: String,
    key_skipped: String,
    key_fast_mode: String,
}

impl Debug for Tracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Tracker {\n")?;
        f.write_str("  microkv: ***,\n")?;
        f.write_str(&format!("  key_raw: {}\n", self.key_raw))?;
        f.write_str(&format!("  key_curt: {}\n", self.key_curt))?;
        f.write_str(&format!("  key_finish: {}\n", self.key_finish))?;
        f.write_str(&format!("  key_running: {}\n", self.key_running))?;
        f.write_str(&format!("  key_next: {}\n", self.key_next))?;
        f.write_str(&format!("  key_skipped: {}\n", self.key_skipped))?;
        f.write_str(&format!("  key_fast_mode: {}\n", self.key_fast_mode))?;
        f.write_str("}")?;
        Ok(())
    }
}

impl Tracker {
    pub fn new(microkv: NamespaceMicroKV, key: impl AsRef<str>) -> Self {
        let key = key.as_ref();
        Self {
            microkv,
            key_raw: key.to_string(),
            key_curt: format!("{}.current", key),
            key_finish: format!("{}.finish", key),
            key_running: format!("{}.running", key),
            key_next: format!("{}.next", key),
            key_skipped: format!("{}.skipped", key),
            key_fast_mode: format!("{}.fast_mode", key),
        }
    }
}

impl Tracker {
    fn read_bool(&self, key: impl AsRef<str>) -> anyhow::Result<bool> {
        let value = self
            .microkv
            .get_as(key.as_ref())?
            .map(|v: String| v.to_lowercase());
        if let Some(v) = value {
            let v = v.trim();
            let value_bool = v == "true" || v == "1";
            return Ok(value_bool);
        }
        Ok(false)
    }
}

impl Tracker {
    pub fn is_running(&self) -> anyhow::Result<bool> {
        self.read_bool(&self.key_running)
    }

    pub fn is_fast_mode(&self) -> anyhow::Result<bool> {
        self.read_bool(&self.key_fast_mode)
    }

    pub fn enable_fast_mode(&self) -> anyhow::Result<()> {
        self.microkv.put(&self.key_fast_mode, &String::from("true"))?;
        Ok(())
    }

    pub fn stop_running(&self) -> anyhow::Result<()> {
        self.microkv.put(&self.key_running, &String::from("false"))?;
        Ok(())
    }

    pub fn start_running(&self) -> anyhow::Result<()> {
        self.microkv.put(&self.key_running, &String::from("true"))?;
        Ok(())
    }

    pub fn reset_current(&self) -> anyhow::Result<()> {
        let finish: Option<usize> = self.microkv.get_as(&self.key_finish)?;
        if let Some(finish) = finish {
            self.microkv.put(&self.key_curt, &finish)?;
        }
        Ok(())
    }

    pub fn flush_current(&self, to: usize) -> anyhow::Result<()> {
        self.microkv.put(&self.key_curt, &to)?;
        Ok(())
    }

    pub async fn next(&self) -> anyhow::Result<usize> {
        let is_running = self.is_running()?;
        if !is_running {
            loop {
                let secs = 3;
                log::warn!(
                    "The track key [{}] isn't running (value is not `true`), wait {} seconds check again.",
                    &self.key_running,
                    secs
                );
                tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
                if self.is_running()? {
                    break;
                }
            }
        }
        let next: Option<String> = self.microkv.get_as(&self.key_next)?;
        if next.is_none() {
            let key = if self.is_fast_mode()? {
                &self.key_curt
            } else {
                &self.key_finish
            };
            let curt: usize = self.microkv.get_as(key)?.unwrap_or(0);
            let next = curt + 1;
            self.microkv.put(&self.key_curt, &next)?;
            return Ok(next);
        }
        let mut plan_blocks = parse_blocks_from_text(next.unwrap())?;
        let next = *plan_blocks.first().unwrap_or(&1);
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

    pub fn finish(&self, block: usize) -> anyhow::Result<()> {
        self.microkv.put(&self.key_finish, &block)?;
        Ok(())
    }

    pub fn skip(&self, block: usize) -> anyhow::Result<()> {
        let skipped: Option<String> = self.microkv.get_as(&self.key_skipped)?;
        match skipped {
            Some(v) => self
                .microkv
                .put(&self.key_skipped, &format!("{},{}", v, block))?,
            None => self.microkv.put(&self.key_skipped, &format!("{}", block))?,
        }
        self.finish(block)?;
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

    pub fn queue(&self, blocks: Vec<usize>) -> anyhow::Result<()> {
        let value: String = blocks.iter().join(",");
        self.microkv.put(&self.key_next, &value)?;
        Ok(())
    }
}

fn parse_blocks_from_text(text: String) -> anyhow::Result<Vec<usize>> {
    let text = text.trim();
    let mut blocks = vec![];
    if text.starts_with('[') && text.ends_with(']') {
        blocks = serde_json::from_str(text)?;
    } else {
        let arrs = text.split(',').collect::<Vec<&str>>();
        for item in arrs {
            let item = item.trim();
            if let Ok(v) = item.parse::<usize>() {
                blocks.push(v);
            }
        }
    }
    Ok(blocks)
}
