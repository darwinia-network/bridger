use getset::{Getters, Setters};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub")]
pub struct TokenGenerateCond {
	remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub")]
pub struct TokenRemoveCond {
	token: String,
}
