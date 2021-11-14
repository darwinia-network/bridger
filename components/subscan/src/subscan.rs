use reqwest::{Client, RequestBuilder};

use crate::types::ExtrinsicsData;
use crate::types::OpenPrice;
use crate::types::SubscanResponse;

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
    pub fn endpoint(mut self, endpoint: impl AsRef<str>) -> Self {
        self.endpoint = endpoint.as_ref().to_string();
        self
    }

    pub fn token(mut self, token: impl AsRef<str>) -> Self {
        self.token = token.as_ref().to_string();
        self
    }
}

impl Subscan {
    async fn _post<T: serde::de::DeserializeOwned>(
        &self,
        api: impl AsRef<str>,
        data_json_string: impl AsRef<str>,
    ) -> anyhow::Result<T> {
        let api = format!("{}{}", self.endpoint, api.as_ref());
        log::trace!(target: "subscan", "POST {} WITH DATA: \n{}", api, data_json_string.as_ref());
        Ok(self
            .http
            .post(api)
            .header("X-API-Key", &self.token)
            .header("Content-Type", "application/json")
            .json(data_json_string.as_ref())
            .send()
            .await?
            .json()
            .await?)
    }

    pub fn _endpoint(&self) -> &String {
        &self.endpoint
    }

    // https://docs.api.subscan.io/#extrinsics
    pub async fn extrinsics(
        &self,
        page: u32,
        row: u32,
    ) -> anyhow::Result<SubscanResponse<ExtrinsicsData>> {
        let data = format!(r#"{{"row": {},"page": {}}}"#, row, page);
        self._post("/api/scan/extrinsics", data).await
    }

    // https://docs.api.subscan.io/#price
    pub async fn price(&self, time: u64) -> anyhow::Result<SubscanResponse<OpenPrice>> {
        let data = format!(r#"{{"time": {}}}"#, time);
        self._post("/api/open/price", data).await
    }
}
