use bridge_standard::error::StandardError;
use hyper::{Body, Request};

pub async fn deserialize_body<T: serde::de::DeserializeOwned>(
    req: &mut Request<Body>,
) -> anyhow::Result<T> {
    let body = req.body_mut();
    match hyper::body::to_bytes(body).await {
        Ok(bytes) => {
            let bytes = bytes.to_vec();
            if bytes.is_empty() {
                Err(StandardError::Api("The body is required".to_string()))?
            }
            match serde_json::from_slice::<T>(bytes.as_slice()) {
                Ok(body) => Ok(body),
                Err(e) => Err(StandardError::Api(format!(
                    "Failed to deserialize body: {}",
                    e
                )))?,
            }
        }
        Err(_e) => Err(StandardError::Api(
            "Failed to parse request body".to_string(),
        ))?,
    }
}
