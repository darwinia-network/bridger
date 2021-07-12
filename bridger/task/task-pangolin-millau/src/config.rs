use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PangolinMillauConfig {}

impl PangolinMillauConfig {
    pub fn store<S: AsRef<str>>(&self, cell_name: S) -> anyhow::Result<()> {
        let _name = cell_name.as_ref();
        Ok(())
    }
    pub fn template() -> Self {
        Self {}
    }
}
