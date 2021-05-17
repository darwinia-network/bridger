use getset::{Getters, Setters};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, Getters, Setters)]
#[getset(get = "pub")]
pub struct TokenGenerateCond {
	remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, Getters, Setters)]
#[getset(get = "pub")]
pub struct TokenRemoveCond {
	token: String,
}
