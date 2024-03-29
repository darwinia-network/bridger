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
    match data.get(&index) {
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
    }
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
    prefix_with_relation_and_others(mark, first, second, relation, Vec::<String>::new())
}

pub fn prefix_with_relation_and_others(
    mark: impl AsRef<str>,
    first: impl AsRef<str>,
    second: impl AsRef<str>,
    relation: impl AsRef<str>,
    others: Vec<impl AsRef<str>>,
) -> String {
    let chain = format!("{}{}{}", first.as_ref(), relation.as_ref(), second.as_ref());
    let mut args = vec![chain];
    let mut o = others
        .iter()
        .map(|v| v.as_ref().to_string())
        .collect::<Vec<String>>();
    args.append(&mut o);
    prefix_multi(mark, args)
}

/// log prefix
pub fn prefix_with_bridge(
    mark: impl AsRef<str>,
    source: impl AsRef<str>,
    target: impl AsRef<str>,
) -> String {
    prefix_with_bridge_and_others(mark, source, target, Vec::<String>::new())
}

pub fn prefix_with_bridge_and_others(
    mark: impl AsRef<str>,
    source: impl AsRef<str>,
    target: impl AsRef<str>,
    others: Vec<impl AsRef<str>>,
) -> String {
    prefix_with_relation_and_others(mark, source, target, ">", others)
}
