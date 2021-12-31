use crate::types::ExtrinsicsData;
use crate::types::OpenPrice;
use crate::types::SubscanResponse;

#[derive(Clone, Debug)]
pub struct Subscan {
    /// HTTP Client
    http: reqwest::blocking::Client,
    endpoint: String,
    token: String,
}

impl Subscan {
    pub fn new(http: reqwest::blocking::Client, endpoint: String, token: String) -> Self {
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
    ) -> color_eyre::Result<T> {
        let api = format!("{}{}", self.endpoint, api.as_ref());
        tracing::trace!(target: "component-subscan", "POST {} ---> {}", api, data_json_string.as_ref());
        let data = serde_json::from_str::<serde_json::Value>(data_json_string.as_ref())?;
        let value = self
            .http
            .post(api)
            .header("X-API-Key", &self.token)
            .header("Content-Type", "application/json")
            .json(&data)
            .send()?
            .text()?;
        tracing::trace!(target: "component-subscan", "<--- {}", value);
        Ok(serde_json::from_str(&value)?)
    }

    pub fn _endpoint(&self) -> &String {
        &self.endpoint
    }

    // https://docs.api.subscan.io/#extrinsics
    pub async fn extrinsics(
        &self,
        page: u32,
        row: u32,
    ) -> color_eyre::Result<SubscanResponse<ExtrinsicsData>> {
        let data = format!(r#"{{"row": {},"page": {}, "signed": "signed"}}"#, row, page);
        self._post("/api/scan/extrinsics", data).await
    }

    // https://docs.api.subscan.io/#price
    pub async fn price(&self, time: u64) -> color_eyre::Result<SubscanResponse<OpenPrice>> {
        let data = format!(r#"{{"time": {}}}"#, time);
        self._post("/api/open/price", data).await
    }
}
