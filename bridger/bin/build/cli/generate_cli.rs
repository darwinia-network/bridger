use std::path::PathBuf;

use crate::generate::types::cli::{GenerateTaskWrapper, Task};

#[derive(Debug)]
pub struct GenerateCli {
    config: PathBuf,
    output: PathBuf,
}

impl GenerateCli {
    pub fn new(config: PathBuf, output: PathBuf) -> Self {
        Self { config, output }
    }
}

impl GenerateCli {
    pub fn generate(&self) -> anyhow::Result<()> {
        let config = std::fs::read_to_string(&self.config)?;
        let wrapper: GenerateTaskWrapper = serde_yaml::from_str(&config)?;
        if !self.output.exists() {
            std::fs::create_dir_all(&self.output)?;
        }
        println!("{:?}", wrapper);
        let tasks = wrapper.tasks;
        for task in tasks {
            self.build_struct_task(task);
        }

        Ok(())
    }

    fn build_struct_task(&self, task: Task) {}
}
