pub struct Component {}

impl Component {
    pub fn cache<S: AsRef<str>>(_name: S) -> anyhow::Result<()> {
        Ok(())
    }
}
