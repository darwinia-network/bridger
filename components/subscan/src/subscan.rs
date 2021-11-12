use reqwest::{Client, RequestBuilder};

use crate::types::common::SubscanResponse;
use crate::types::general::ExtrinsicsData;

#[derive(Clone, Debug)]
pub struct Subscan {
    /// HTTP Client
    http: Client,
    endpoint: String,
    token: String,
}

impl Subscan {
    pub fn new(http: Client, endpoint: String, token: String) -> Self {
        Self {
            http,
            endpoint,
            token,
        }
    }
}

impl Subscan {
    fn request(&self, api: impl AsRef<str>) -> RequestBuilder {
        let api = format!("{}{}", self.endpoint, api.as_ref());
        self.http
            .post(api)
            .header("X-API-Key", &self.token)
            .header("Content-Type", "application/json")
    }

    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }

    pub fn extrinsics(
        &self,
        page: u32,
        row: u32,
    ) -> anyhow::Result<SubscanResponse<ExtrinsicsData>> {
        let data = format!(r#"{{"row": {},"page": {}}}"#, row, page);
        Ok(self
            .request("/api/scan/extrinsics")
            .json(&data)
            .send()
            .await?
            .json()?)
    }
}
