use serde_json::Value;

pub fn best_view_option(value: Option<&Value>) -> anyhow::Result<String> {
    match value {
        Some(v) => best_view(v),
        None => Ok("null".to_string()),
    }
}
pub fn best_view(value: &Value) -> anyhow::Result<String> {
    if value.is_string() {
        return Ok(value.as_str().unwrap_or_else(|| "").to_string());
    }
    Ok(serde_json::to_string_pretty(value)?)
}
