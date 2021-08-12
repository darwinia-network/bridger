use bridge_traits::error::StandardError;
use microkv::namespace::NamespaceMicroKV;
use serde_json::Value;

pub fn best_view_option(value: Option<&Value>) -> anyhow::Result<String> {
    match value {
        Some(v) => best_view(v),
        None => Ok("null".to_string()),
    }
}

pub fn best_view(value: &Value) -> anyhow::Result<String> {
    if value.is_string() {
        return Ok(value.as_str().unwrap_or("").to_string());
    }
    Ok(serde_json::to_string_pretty(value)?)
}

pub fn spec_serialize_value(
    microkv: &NamespaceMicroKV,
    key: String,
    value: String,
) -> anyhow::Result<()> {
    if !key.contains("::") {
        microkv.put(key, &value)?;
        return Ok(());
    }
    let mut split = key.split("::").collect::<Vec<&str>>();
    let key = split.first().unwrap().to_string();
    split.remove(0);
    let value_type: String = split.join("::");
    match &value_type[..] {
        "String" | "string" | "str" => {
            microkv.put(key, &value)?;
        }
        "isize" | "i8" | "i16" | "i32" | "i64" | "i128" => {
            let value = value.parse::<isize>()?;
            microkv.put(key, &value)?;
        }
        "usize" | "u8" | "u16" | "u32" | "u64" | "u128" => {
            let value = value.parse::<usize>()?;
            microkv.put(key, &value)?;
        }
        "f32" | "f64" => {
            let value = value.parse::<f64>()?;
            microkv.put(key, &value)?;
        }
        "bool" => {
            let value = value.parse::<bool>()?;
            microkv.put(key, &value)?;
        }
        _ => {
            return Err(
                StandardError::Api(format!("Not support value type: {}", value_type)).into(),
            );
        }
    }
    Ok(())
}
