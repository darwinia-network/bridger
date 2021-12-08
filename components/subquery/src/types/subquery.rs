use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub(crate) struct EmptyQueryVar;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct QueryTransactionsVars {
    pub(crate) from: u64,
    pub(crate) first: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubqueryResponse<T> {
    data: HashMap<String, DataWrapper<T>>,
}

impl<T> SubqueryResponse<T> {
    pub fn data(&self) -> &HashMap<String, DataWrapper<T>> {
        &self.data
    }

    pub fn data_by_key(&self, key: impl AsRef<str>) -> Option<&DataWrapper<T>> {
        self.data.get(key.as_ref())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataWrapper<T> {
    pub nodes: Vec<T>,
}
