use getset::{Getters, Setters};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters, Setters)]
#[getset(get = "pub")]
pub struct ChainRemoveCond {
	name: String,
}
