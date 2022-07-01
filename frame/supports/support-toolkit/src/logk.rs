use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use pad::{Alignment, PadStr};

static MAX_LEN: usize = 26;

static LEN_ARG: Lazy<Mutex<HashMap<u32, usize>>> = Lazy::new(|| {
    let map = HashMap::new();
    Mutex::new(map)
});

fn get_len_arg_with_def(index: u32, def: usize) -> usize {
    let mut data = LEN_ARG.lock().unwrap();
    let size = match data.get(&index) {
        Some(v) => {
            if def > *v {
                data.insert(index, def);
                def
            } else {
                *v
            }
        }
        None => {
            let len = if def > MAX_LEN { MAX_LEN } else { def };
            data.insert(index, len);
            len
        }
    };
    drop(data);
    size
}

/// prefix multiple aruments
pub fn prefix_multi(mark: impl AsRef<str>, args: Vec<impl AsRef<str>>) -> String {
    let mark = mark.as_ref();
    let len_mark = get_len_arg_with_def(0, mark.len());
    let mut prefix = vec![mark.pad(len_mark, ' ', Alignment::Left, true)];
    for (ix, v) in args.iter().enumerate() {
        let pix = ix as u32 + 1;
        let arg = v.as_ref();
        let len_arg = get_len_arg_with_def(pix, arg.len());
        prefix.push(arg.pad(len_arg, ' ', Alignment::Middle, true))
    }
    format!("[{}]", prefix.join("] ["))
}

pub fn prefix_with_relation(
    mark: impl AsRef<str>,
    first: impl AsRef<str>,
    second: impl AsRef<str>,
    relation: impl AsRef<str>,
) -> String {
    let chain = format!("{}{}{}", first.as_ref(), relation.as_ref(), second.as_ref());
    prefix_multi(mark, vec![chain])
}

/// log prefix
pub fn prefix_with_bridge(
    mark: impl AsRef<str>,
    source: impl AsRef<str>,
    target: impl AsRef<str>,
) -> String {
    prefix_with_relation(mark, source, target, ">")
}
