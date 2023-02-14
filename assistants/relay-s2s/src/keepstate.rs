use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Mutex;

use once_cell::sync::Lazy;

static LAST_RELAYED_NONCE: Lazy<Mutex<HashMap<String, u64>>> = Lazy::new(|| {
    let map = HashMap::new();
    Mutex::new(map)
});

static RECENTLY_JUSTIFICATIONS: Lazy<Mutex<HashMap<&str, VecDeque<sp_core::Bytes>>>> =
    Lazy::new(|| {
        let map = HashMap::new();
        Mutex::new(map)
    });

pub fn get_last_delivery_relayed_nonce(chain: &str) -> Option<u64> {
    let data = LAST_RELAYED_NONCE.lock().unwrap();
    data.get(&format!("{chain}-delivery")).cloned()
}

pub fn set_last_delivery_relayed_nonce(chain: &str, nonce: u64) {
    let mut data = LAST_RELAYED_NONCE.lock().unwrap();
    data.insert(format!("{chain}-delivery"), nonce);
}

pub fn get_last_receiving_relayed_nonce(chain: &str) -> Option<u64> {
    let data = LAST_RELAYED_NONCE.lock().unwrap();
    data.get(&format!("{chain}-receiving")).cloned()
}

pub fn set_last_receiving_relayed_nonce(chain: &str, nonce: u64) {
    let mut data = LAST_RELAYED_NONCE.lock().unwrap();
    data.insert(format!("{chain}-receiving"), nonce);
}

pub fn set_recently_justification(chain: &'static str, justification: sp_core::Bytes) {
    let mut data = RECENTLY_JUSTIFICATIONS.lock().unwrap();
    let queue = data
        .entry(chain)
        .or_insert_with(|| VecDeque::with_capacity(10));
    queue.push_back(justification);
    if queue.len() >= 10 {
        queue.pop_front();
    }
}

pub(crate) fn get_recently_justification(chain: &'static str) -> Option<sp_core::Bytes> {
    let recently_justifications = RECENTLY_JUSTIFICATIONS.lock().unwrap();
    let justification_queue = recently_justifications.get(chain);
    justification_queue.and_then(|v| v.back().cloned())
}
