use microkv::namespace::NamespaceMicroKV;
use std::fmt::{Debug, Formatter};

// discuss: https://github.com/darwinia-network/bridger/issues/284
/// Tracker
/// Block scan status
#[derive(Clone)]
pub struct Tracker {
    microkv: NamespaceMicroKV,
    /// Current scan value, the next value is current+1
    key_current: String,
    /// Planned to execute, after to running the next value is planned+1
    key_planned: String,
    /// Control running
    key_running: String,
}

impl Debug for Tracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Tracker {\n")?;
        f.write_str("  microkv: ***,\n")?;
        f.write_str(&format!("  key_current: {}\n", self.key_current))?;
        f.write_str(&format!("  key_planned: {}\n", self.key_planned))?;
        f.write_str(&format!("  key_running: {}\n", self.key_running))?;
        f.write_str("}")?;
        Ok(())
    }
}

impl Tracker {
    /// Create a new tracker, the key is prefix
    pub fn new(microkv: NamespaceMicroKV, key: impl AsRef<str>) -> Self {
        let key = key.as_ref();
        Self {
            microkv,
            key_current: format!("{}.current", key),
            key_planned: format!("{}.planned", key),
            key_running: format!("{}.running", key),
        }
    }
}

impl Tracker {
    pub fn is_running(&self) -> anyhow::Result<bool> {
        self.read_bool(&self.key_running)
    }

    pub fn stop_running(&self) -> anyhow::Result<()> {
        self.microkv.put(&self.key_running, &false)?;
        Ok(())
    }

    pub fn start_running(&self) -> anyhow::Result<()> {
        self.microkv.put(&self.key_running, &true)?;
        Ok(())
    }

    /// Read current value
    pub async fn current(&self) -> anyhow::Result<usize> {
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

        match self.read_u64(&self.key_planned)? {
            Some(planned) => {
                self.microkv.delete(&self.key_planned)?;
                Ok(planned as usize)
            }
            None => {
                let current: usize = self.microkv.get_as(&self.key_current)?.unwrap_or(0);
                Ok(current)
            }
        }
    }

    /// Read the next value
    /// Generally the value is current+1, but if the planned have value, will use planned+1
    pub async fn next(&self) -> anyhow::Result<usize> {
        self.current().await.map(|v| v + 1)
    }

    /// Update current
    pub fn finish(&self, block: usize) -> anyhow::Result<()> {
        self.microkv.put(&self.key_current, &block)?;
        Ok(())
    }

    pub fn planned(&self, block: usize) -> anyhow::Result<()> {
        self.microkv.put(&self.key_planned, &block)?;
        Ok(())
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
                            Ok(Some(t.parse()?))
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
