use microkv::namespace::NamespaceMicroKV;

#[derive(Clone)]
pub struct Tracker {
    microkv: NamespaceMicroKV,
    key_curt: String,
    key_next: String,
}

impl Tracker {
    pub fn new(microkv: NamespaceMicroKV, key: String) -> Self {
        Self {
            microkv,
            key_curt: format!("{}.current", &key),
            key_next: key,
        }
    }
}

impl Tracker {}
