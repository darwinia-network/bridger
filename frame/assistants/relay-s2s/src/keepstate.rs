use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Mutex;

use once_cell::sync::{Lazy, OnceCell};

use crate::error::{RelayError, RelayResult};

static LAST_DELIVERY_RELAYED_NONCE: OnceCell<u64> = OnceCell::new();

static LAST_RECEIVING_RELAYED_NONCE: OnceCell<u64> = OnceCell::new();

static RECENTLY_JUSTIFICATIONS: Lazy<Mutex<HashMap<&str, VecDeque<sp_core::Bytes>>>> =
    Lazy::new(|| {
        let map = HashMap::new();
        Mutex::new(map)
    });

pub fn get_last_delivery_relayed_nonce() -> Option<u64> {
    LAST_DELIVERY_RELAYED_NONCE.get().cloned()
}

pub fn set_last_delivery_relayed_nonce(nonce: u64) -> RelayResult<()> {
    LAST_DELIVERY_RELAYED_NONCE.set(nonce).map_err(|e| {
        RelayError::Custom(format!("Failed to set last delivery relayed nonce: {}", e))
    })
}

pub fn get_last_receiving_relayed_nonce() -> Option<u64> {
    LAST_RECEIVING_RELAYED_NONCE.get().cloned()
}

pub fn set_last_receiving_relayed_nonce(nonce: u64) -> RelayResult<()> {
    LAST_RECEIVING_RELAYED_NONCE.set(nonce).map_err(|e| {
        RelayError::Custom(format!("Failed to set last delivery relayed nonce: {}", e))
    })
}

pub fn set_recently_justification(chain: &'static str, justification: sp_core::Bytes) {
    let mut data = RECENTLY_JUSTIFICATIONS.lock().unwrap();
    let queue = data
        .entry(chain)
        .or_insert_with(|| VecDeque::with_capacity(100));
    queue.push_back(justification);
    if queue.len() >= 100 {
        queue.pop_front();
    }
}

pub(crate) fn get_recently_justification(chain: &'static str) -> Option<sp_core::Bytes> {
    let recently_justifications = RECENTLY_JUSTIFICATIONS.lock().unwrap();
    let justification_queue = recently_justifications.get(chain);
    justification_queue.map(|v| v.back().cloned()).flatten()
}
