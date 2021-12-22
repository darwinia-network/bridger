use crate::command::output;
use crate::external;

pub fn exec_list() -> color_eyre::Result<()> {
    let (_, binaries) = external::helpers::list_externals(None)?;
    binaries.iter().for_each(output::output_text);
    Ok(())
}
