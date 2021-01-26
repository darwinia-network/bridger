use crate::{
    HeaderMMR,
    Darwinia,
};

use crate::error::{
    Result,
};

use codec::Encode;
    
use substrate_subxt::{
    sp_core::H256,
};

#[derive(Encode)]
struct _S<_1, _2, _3, _4>
where
	_1: Encode,
	_2: Encode,
	_3: Encode,
	_4: Encode,
{
	_1: _1, // spec name
	_2: _2, // op code, mmr root: 0x479fbdf9, next authorities: 0xb4bcf497
	#[codec(compact)]
	_3: _3, // block_number or term
	_4: _4, // mmr_root or next authorities
}

/// Dawrinia API
pub struct Darwinia2Ethereum {
    /// darwinia client
    pub darwinia: Darwinia,
}

impl Darwinia2Ethereum {
	pub fn new(darwinia: Darwinia) -> Self {
        Self { darwinia }
    }

    /// header mmr proof
    pub async fn get_headermmr_genproof(&self, member_leaf: u64, last_leaf: u64, hash: H256) -> Result<Option<HeaderMMR>> {
        return self
            .darwinia
            .rpc
            .header_mmr(member_leaf, last_leaf, hash)
            .await
    }

    /// construct mmr root message
	pub fn construct_mmr_root_message(
		spec_name: String,
		block_number: u32,
		mmr_root: H256,
	) -> Vec<u8> {
		let op_code: [u8; 4] = [71, 159, 189, 249];
		debug!(
			"Infos to construct mmr_root message: {}, {}, {}, {:?}",
			spec_name,
			hex::encode(&op_code),
			block_number,
			mmr_root
		);
		// scale encode & sign
		let message = _S {
			_1: spec_name,
			_2: op_code,
			_3: block_number,
			_4: mmr_root,
		};
		let encoded: &[u8] = &message.encode();
		encoded.to_vec()
	}
}
