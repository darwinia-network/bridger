use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Mutex;

use once_cell::sync::Lazy;

static LAST_RELAYED_NONCE: Lazy<Mutex<HashMap<&str, u64>>> = Lazy::new(|| {
    let map = HashMap::new();
    Mutex::new(map)
});

static RECENTLY_JUSTIFICATIONS: Lazy<Mutex<HashMap<&str, VecDeque<sp_core::Bytes>>>> =
    Lazy::new(|| {
        let map = HashMap::new();
        Mutex::new(map)
    });

pub fn get_last_delivery_relayed_nonce() -> Option<u64> {
    let data = LAST_RELAYED_NONCE.lock().unwrap();
    let nonce = data.get("delivery").cloned();
    drop(data);
    nonce
}

pub fn set_last_delivery_relayed_nonce(nonce: u64) {
    let mut data = LAST_RELAYED_NONCE.lock().unwrap();
    data.insert("delivery", nonce);
    drop(data);
}

pub fn get_last_receiving_relayed_nonce() -> Option<u64> {
    let data = LAST_RELAYED_NONCE.lock().unwrap();
    let nonce = data.get("receiving").cloned();
    drop(data);
    nonce
}

pub fn set_last_receiving_relayed_nonce(nonce: u64) {
    let mut data = LAST_RELAYED_NONCE.lock().unwrap();
    data.insert("receiving", nonce);
    drop(data);
}

pub fn set_recently_justification(chain: &'static str, justification: sp_core::Bytes) {
    let mut data = RECENTLY_JUSTIFICATIONS.lock().unwrap();
    let queue = data
        .entry(chain)
        .or_insert_with(|| VecDeque::with_capacity(100));
    queue.push_back(justification);
    if queue.len() >= 10 {
        queue.pop_front();
    }
    drop(data);
}

pub(crate) fn get_recently_justification(chain: &'static str) -> Option<sp_core::Bytes> {
    let recently_justifications = RECENTLY_JUSTIFICATIONS.lock().unwrap();
    let justification_queue = recently_justifications.get(chain);
    let justification = justification_queue.map(|v| v.back().cloned()).flatten();
    drop(recently_justifications);
    justification
}
