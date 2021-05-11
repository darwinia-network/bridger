use getset::{Getters, Setters};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, Default, TypedBuilder, Getters, Setters)]
#[getset(get = "pub")]
pub struct ChainRemoveCond {
	name: String,
}
