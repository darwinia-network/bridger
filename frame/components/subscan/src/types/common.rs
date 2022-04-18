use serde::{Deserialize, Serialize};

use crate::{SubscanComponentError, SubscanComponentResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscanResponse<T: Clone> {
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T: Clone> SubscanResponse<T> {
    pub fn code(&self) -> i32 {
        self.code
    }

    pub fn data(&self) -> SubscanComponentResult<Option<T>> {
        if self.code != 0 {
            return Err(SubscanComponentError::WrongResponse(
                self.code,
                self.message.clone(),
            ));
        }
        Ok(self.data.clone())
    }
}
