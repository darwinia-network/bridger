use pad::{Alignment, PadStr};

/// log prefix
pub fn log_prefix(
    mark: impl AsRef<str>,
    source: impl AsRef<str>,
    target: impl AsRef<str>,
) -> String {
    let mark = mark.as_ref().pad(9, ' ', Alignment::Left, true);
    let bridge = format!("{}>{}", source.as_ref(), target.as_ref());
    let len = bridge.len() + 2;
    let bridge = bridge.pad(len, ' ', Alignment::Middle, true);
    format!("[{}] [{}]", mark, bridge)
}
