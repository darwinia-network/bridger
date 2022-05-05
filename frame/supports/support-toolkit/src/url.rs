use crate::error::{TkError, TkResult};

/// Correct endpoint, add port for endpoint
pub fn correct_endpoint(url: impl AsRef<str>) -> TkResult<String> {
    let url = url.as_ref();
    if url.starts_with("ws://") || url.starts_with("wss://") {
        let is_ssl = url.starts_with("wss://");
        let fixed_url = url.replace("ws://", "").replace("wss://", "");
        let correct_url = common_correct_endpoint(is_ssl, fixed_url)?;
        let better_url = format!("{}{}", if is_ssl { "wss://" } else { "ws://" }, correct_url);
        return Ok(better_url);
    }
    if url.starts_with("http://") || url.starts_with("https://") {
        let is_ssl = url.starts_with("https://");
        let fixed_url = url.replace("http://", "").replace("https://", "");
        let correct_url = common_correct_endpoint(is_ssl, fixed_url)?;
        let better_url = format!(
            "{}{}",
            if is_ssl { "https://" } else { "http://" },
            correct_url
        );
        return Ok(better_url);
    }
    Err(TkError::Custom(format!("Wrong url: {}", url)))
}

fn common_correct_endpoint(is_ssl: bool, fixed: impl AsRef<str>) -> TkResult<String> {
    let mut parts = fixed.as_ref().split('/').collect::<Vec<&str>>();
    let origin_host = parts
        .first()
        .ok_or_else(|| TkError::Custom("Bad url".to_string()))?;
    let mut better_host = origin_host.to_string();
    if !origin_host.contains(':') {
        let port = if is_ssl { 443 } else { 80 };
        better_host = format!("{}:{}", better_host, port);
    }
    parts.remove(0);
    Ok(format!("{}{}", better_host, parts.join("/")))
}
