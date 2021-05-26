use serde::Deserialize;
use std::str::FromStr;
use substrate_subxt::{
	sp_core::H256,
	sp_runtime::traits::{BlakeTwo256, Hash},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeaderMMRRpc {
	mmr_size: String,
	proof: String,
}

/// Header MMR Info
#[derive(Debug)]
pub struct HeaderMMR {
	/// block number
	pub block: u64,
	/// block hash
	pub hash: H256,
	/// mmr size of header
	pub mmr_size: u64,
	/// mmr proof
	pub proof: Vec<String>,
}

/// formated mmr header
#[derive(Debug)]
pub struct FormatedMMR {
	/// block number
	pub block: u64,
	/// block hash
	pub hash: H256,
	/// mmr size of header
	pub mmr_size: u64,
	/// peaks in mmr proof
	pub peaks: Vec<String>,
	/// siblings in mmr proof
	pub siblings: Vec<String>,
}

impl From<HeaderMMRRpc> for Option<HeaderMMR> {
	fn from(that: HeaderMMRRpc) -> Self {
		let mmr_size = that.mmr_size.parse::<u64>();
		if let Err(err) = mmr_size {
			println!("parse mmr size failed {}", err);
			return None;
		}
		//let proof = serde_json::from_str(&self.proof);
		let trim: &[_] = &['[', ']'];
		let proof: Vec<String> = that
			.proof
			.trim_matches(trim)
			.split(',')
			.map(|s| String::from(s.trim()))
			.collect();
		Some(HeaderMMR {
			block: 0,
			hash: H256::from_str(
				"0000000000000000000000000000000000000000000000000000000000000000",
			)
			.unwrap(),
			mmr_size: mmr_size.unwrap(),
			proof,
		})
	}
}

impl From<HeaderMMR> for Option<FormatedMMR> {
	fn from(that: HeaderMMR) -> Self {
		let (peaks, siblings) = convert(that.mmr_size, that.block, that.hash, that.proof);
		Some(FormatedMMR {
			block: that.block,
			hash: that.hash,
			mmr_size: that.mmr_size,
			peaks,
			siblings,
		})
	}
}

fn get_peaks(mmr_size: u64) -> Vec<u64> {
	let mut pos_s = Vec::new();
	let (mut height, mut pos) = left_peak_height_pos(mmr_size);
	pos_s.push(pos);
	while height > 0 {
		let peak = match get_right_peak(height, pos, mmr_size) {
			Some(peak) => peak,
			None => break,
		};
		height = peak.0;
		pos = peak.1;
		pos_s.push(pos);
	}
	pos_s
}

fn get_right_peak(mut height: u32, mut pos: u64, mmr_size: u64) -> Option<(u32, u64)> {
	pos += (2 << height) - 1;
	while pos > mmr_size - 1 {
		if height == 0 {
			return None;
		}
		pos -= 2 << (height - 1);
		height -= 1;
	}
	Some((height, pos))
}

fn left_peak_height_pos(mmr_size: u64) -> (u32, u64) {
	let mut height = 1;
	let mut prev_pos = 0;
	let mut pos = (1 << (height + 1)) - 2;
	while pos < mmr_size {
		height += 1;
		prev_pos = pos;
		pos = (1 << (height + 1)) - 2;
	}
	(height - 1, prev_pos)
}

fn leaf_index_to_mmr_size(index: u64) -> u64 {
	let leaves_count = index + 1;
	let peak_count = leaves_count.count_ones() as u64;
	2 * leaves_count - peak_count
}

fn pos_height_in_tree(mut pos: u64) -> u32 {
	pos += 1;
	fn all_ones(num: u64) -> bool {
		num != 0 && num.count_zeros() == num.leading_zeros()
	}
	fn jump_left(pos: u64) -> u64 {
		let bit_length = 64 - pos.leading_zeros();
		let most_significant_bits = 1 << (bit_length - 1);
		pos - (most_significant_bits - 1)
	}

	while !all_ones(pos) {
		pos = jump_left(pos)
	}

	64 - pos.leading_zeros() - 1
}

fn get_merkle_root(pos: u64, peak_pos: u64, item: H256, proofs: Vec<H256>) -> H256 {
	let mut height = 0;
	let mut parent = pos;
	let mut proofiter = proofs.iter();
	let mut parent_item = item;
	while parent != peak_pos {
		let next_height = pos_height_in_tree(parent + 1);
		let sibling_offset = (2 << height) - 1;

		let (sibling_pos, parent_pos) = {
			if next_height > height {
				(parent - sibling_offset, parent + 1)
			} else {
				(parent + sibling_offset, parent + (2 << height))
			}
		};
		let sibling_item = {
			if parent == sibling_pos {
				parent_item
			} else {
				*proofiter.next().unwrap()
			}
		};
		let encodable = {
			if next_height > height {
				(sibling_item, parent_item)
			} else {
				(parent_item, sibling_item)
			}
		};
		parent_item = BlakeTwo256::hash_of(&encodable);
		parent = parent_pos;
		height += 1;
	}
	parent_item
}

fn convert(
	mmr_size: u64,
	block: u64,
	block_hash: H256,
	mmr_proof: Vec<String>,
) -> (Vec<String>, Vec<String>) {
	let mut res_peaks = Vec::new();
	let peaks = get_peaks(mmr_size);
	let peaksize = peaks.len();
	let pos = leaf_index_to_mmr_size(block) - (block + 1).trailing_zeros() as u64 - 1;
	info!("leaf position is {}", pos);
	let mut mmr_proof_iter = mmr_proof.iter();
	for peak in &peaks {
		if peak < &pos {
			res_peaks.push(String::from(mmr_proof_iter.next().unwrap()));
		} else {
			break;
		}
	}
	let (peak_pos, last, res_siblings) = if res_peaks.len() + 1 == peaksize {
		(
			peaks[res_peaks.len()],
			None,
			mmr_proof[res_peaks.len()..].to_vec(),
		)
	} else {
		(
			peaks[res_peaks.len()],
			Some(&mmr_proof[mmr_proof.len() - 1]),
			mmr_proof[res_peaks.len()..mmr_proof.len() - 1].to_vec(),
		)
	};
	let proof: Vec<H256> = mmr_proof_iter
		.map(|hash| H256::from_str(&hash[2..]).unwrap())
		.collect();
	let root = get_merkle_root(pos, peak_pos, block_hash, proof);
	res_peaks.push(array_bytes::bytes2hex("0x", root));
	if let Some(last_hash) = last {
		res_peaks.push(String::from(last_hash));
	}
	(res_peaks, res_siblings)
}

#[cfg(test)]
mod mmr_convert_test {
	use super::*;
	#[test]
	fn test_convert() {
		let rpcheader = HeaderMMRRpc {
            mmr_size: String::from("7261431"),
            proof: String::from("[0x100d7b5c325baa1ee0403b9a3f0500c9f1e6a6af796d22f69a5f9326bcaf0768,0x75128e0b7dde0738bc1ec366af6f928d457c25eae091371a0da34261868933ed,0xc046be6fc40bf641c5c00c9b0e5e6c79fc8b1dcc414d0983db5284973503e480,0x3d3dd94c70124c5895648a17eee5d0aba7178303dfa824808787a9ffd3115537,0xf73f4035df1d3799ed26ed2b0e04df8cd262b4762df83c5bdcf41a44895fd6b4,0xb9c761dc3446b3216fae5d003514513cb9fbcf5853025f088df21438f089b434,0x45b2f9ac26928873802dd124ea08c8bc7fa87460d4c5e73db7679a4e6fcdeca4,0xc4905445f8cac409704f76e708ef2bb12d754e270010eab521af6e599f8ecb43,0x153b1f802819948d79dd7ee4a1e2b107fb2211f90d5ee7307cd03867852367d4,0x6b5ea1569b586be1dff789bca89dd3dd6de5f676d090f27ef0cf61aef28f1965,0x7d988132cd401bfcccf9073db3c355b38f32df277d8f541db3df640c5d773176,0x92d453e2cbe3f414e52163cd16da0e95f34dd4bbb005c3bf604416db20765229,0x6a6f39470e6bc4b9fe72b9ca2c403ea85de92e3461e827f6040f90ad90ff647d,0x8e90e065c02c04983db00e667ead2977af2f28ff386b269904d7730340220238,0xd169bed480658089615bd9d0f1ec1b7118d853f6ff706565c3878557bb7408b0,0x7faad6ff144f22d1a8a612de2f4741e751b882d3edb99211cfe4d5da96de20f8,0xb336184cdf4a308623cf13eb859d95e1a602d2a0b93abf919f6afbbb8eb97586]"),
        };
		let header_mmr: Option<HeaderMMR> = rpcheader.into();
		let mut header = header_mmr.unwrap();
		header.block = 3630712;
		header.hash =
			H256::from_str("5431b121fb0549830b7f33865d290eea566cc1ae83b260c2edc8b44cfc9115d3")
				.unwrap();
		println!("{:?}", header);

		let formated_mmr: Option<FormatedMMR> = header.into();
		if let Some(mmr_proof) = formated_mmr {
			assert_eq!(
				"0x1c7d559a2231da8354399af954f8faa205f410995deba7fd25086dcb663a77ec",
				mmr_proof.peaks[&mmr_proof.peaks.len() - 2]
			);
		}
	}
}
