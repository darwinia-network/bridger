use bridge_traits::error::StandardError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscanResponse<T> {
    code: i32,
    message: String,
    data: T,
}

impl<T> SubscanResponse<T> {
    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn data(&self) -> anyhow::Result<&T> {
        if self.code != 0 {
            return Err(StandardError::Component(format!(
                "Wrong response [{}]: {}",
                self.code, self.message
            ))
            .into());
        }
        Ok(&self.data)
    }
}
