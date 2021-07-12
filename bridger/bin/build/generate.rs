use std::path::Path;

mod cli;
mod types;

pub fn generate() -> anyhow::Result<()> {
    generate_cli()?;
    Ok(())
}

fn generate_cli() -> anyhow::Result<()> {
    let path_config = Path::new(".").join("resources/rest-cli.yml");
    let path_output = Path::new(".").join("src/output");
    let generate = cli::GenerateCli::new(path_config.into(), path_output.into());
    generate.generate()
}
