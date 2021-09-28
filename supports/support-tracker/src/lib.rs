use itertools::Itertools;
use microkv::namespace::NamespaceMicroKV;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Tracker {
    microkv: NamespaceMicroKV,
    key_current: String,
    key_planned: String,
}

impl Debug for Tracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Tracker {\n")?;
        f.write_str("  microkv: ***,\n")?;
        f.write_str(&format!("  key_current: {}\n", self.key_current))?;
        f.write_str(&format!("  key_planned: {}\n", self.key_planned))?;
        f.write_str("}")?;
        Ok(())
    }
}

impl Tracker {
    pub fn new(microkv: NamespaceMicroKV, key: impl AsRef<str>) -> Self {
        let key = key.as_ref();
        Self {
            microkv,
            key_current: format!("{}.current", key),
            key_planned: format!("{}.planned", key),
        }
    }
}

impl Tracker {
    /// Read bool value by a key
    fn read_bool(&self, key: impl AsRef<str>) -> anyhow::Result<bool> {
        let value = self
            .microkv
            .get(key.as_ref())?
            .unwrap_or(serde_json::Value::Bool(false));
        if value.is_boolean() {
            return Ok(value.as_bool().unwrap_or(false));
        }
        if value.is_string() {
            let text = value.as_str().unwrap_or("false");
            return Ok(text == "true" || text == "1");
        }
        Ok(false)
    }

    fn read_u64(&self, key: impl AsRef<str>) -> anyhow::Result<Option<u64>> {
        let value = self.microkv.get(key.as_ref())?;
        match value {
            Some(v) => {
                if v.is_number() {
                    return Ok(v.as_u64());
                }
                if v.is_boolean() {
                    return Ok(v.as_bool().map(|b| if b { 1 } else { 0 }));
                }
                if v.is_string() {
                    return match v.as_str() {
                        Some(t) => {
                            let t = t.trim();
                            Ok(Some((t.parse::<u64>()?)))
                        }
                        None => Ok(None),
                    };
                }
                Ok(None)
            }
            None => Ok(None),
        }
    }
}

impl Tracker {
    pub fn is_running(&self) -> anyhow::Result<bool> {
        self.read_bool(&self.key_running)
    }

    pub fn stop_running(&self) -> anyhow::Result<()> {
        self.microkv
            .put(&self.key_running, &String::from("false"))?;
        Ok(())
    }

    pub fn start_running(&self) -> anyhow::Result<()> {
        self.microkv.put(&self.key_running, &String::from("true"))?;
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

    pub fn planned(&self, blocks: Vec<usize>) -> anyhow::Result<()> {
        let value: String = blocks.iter().join(",");
        self.microkv.put(&self.key_next, &value)?;
        Ok(())
    }
}
