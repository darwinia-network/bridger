use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscanResponse<T> {
    pub code: i32,
    pub data: T,
}
