use std::process;

mod initialize;

fn main() -> color_eyre::Result<()> {
    initialize::init()?;
    println!("Hello, world!");
    Ok(())
}
