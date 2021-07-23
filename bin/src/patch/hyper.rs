use bridge_traits::error::StandardError;
use hyper::{Body, Request};

pub async fn deserialize_body<T: serde::de::DeserializeOwned>(
    req: &mut Request<Body>,
) -> anyhow::Result<T> {
    let body = req.body_mut();
    match hyper::body::to_bytes(body).await {
        Ok(bytes) => {
            let bytes = bytes.to_vec();
            if bytes.is_empty() {
                return Err(StandardError::Api("The body is required".to_string()).into());
            }
            match serde_json::from_slice::<T>(bytes.as_slice()) {
                Ok(body) => Ok(body),
                Err(e) => {
                    return Err(
                        StandardError::Api(format!("Failed to deserialize body: {}", e)).into(),
                    )
                }
            }
        }
        Err(_e) => Err(StandardError::Api("Failed to parse request body".to_string()).into()),
    }
}
