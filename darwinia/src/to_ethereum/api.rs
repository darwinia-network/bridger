use crate::{
    HeaderMMR,
    Darwinia,
    ToEthereumAccount,
};

use crate::error::{
    Result,
    DarwiniaError,
};

use codec::Encode;
    
use substrate_subxt::{
    sp_core::H256,
};

use primitives::{
    chain::{
        ethereum::{EthereumReceiptProofThing},
        proxy_type::ProxyType,
    },
    frame::{
        ethereum::{
            backing::{
                SyncAuthoritiesChange, SyncAuthoritiesChangeCallExt,
            }
        },
        bridge::relay_authorities::{
            NextTermStoreExt,
            SubmitSignedAuthorities,
            SubmitSignedAuthoritiesCallExt,
            SubmitSignedMmrRoot,
            SubmitSignedMmrRootCallExt,
            AuthoritiesStoreExt,
            AuthoritiesToSignStoreExt,
            MmrRootsToSignStoreExt,
        },
        proxy::ProxyCallExt,
    },
    runtime::{EcdsaAddress, EcdsaMessage},
};

use core::marker::PhantomData;

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

    /// construct_message
    pub fn construct_authorities_message(
        spec_name: String,
        term: u32,
        next_authorities: Vec<EcdsaAddress>,
    ) -> Vec<u8> {
        let op_code: [u8; 4] = [180, 188, 244, 151];
        debug!(
            "Infos to construct eth authorities message: {}, {}, {}, {:?}",
            spec_name,
            hex::encode(&op_code),
            term,
            next_authorities
                .iter()
                .map(|a| hex::encode(&a))
                .collect::<Vec<_>>()
                .join(", ")
        );
        // scale encode & sign
        let message = _S {
            _1: spec_name,
            _2: op_code,
            _3: term,
            _4: next_authorities,
        };
        let encoded: &[u8] = &message.encode();
        encoded.to_vec()
    }

    /// get_current_term
    pub async fn get_current_authority_term(&self) -> Result<u32> {
        Ok(self.darwinia.subxt.next_term(None).await?)
    }

    // use account
    /// sync authorities change from ethereum to darwinia
    pub async fn sync_authorities_change(
        &self,
        account: &ToEthereumAccount,
        proof: EthereumReceiptProofThing,
    ) -> Result<H256> {
        match &account.darwinia_account.real {
            Some(real) => {
                let call = SyncAuthoritiesChange {
                    _runtime: PhantomData::default(),
                    proof,
                };

                let ex = self.darwinia.subxt.encode(call).unwrap();
                Ok(self
                    .darwinia
                    .subxt
                    .proxy(
                        &account.darwinia_account.signer,
                        real.clone(),
                        Some(ProxyType::EthereumBridge),
                        &ex,
                    )
                    .await?)
            }
            None => {
                Ok(self
                    .darwinia
                    .subxt
                    .sync_authorities_change(&account.darwinia_account.signer, proof)
                    .await?)
            }
        }
    }

    /// submit_signed_authorities
    pub async fn ecdsa_sign_and_submit_signed_authorities(
        &self,
        account: &ToEthereumAccount,
        message: EcdsaMessage,
    ) -> Result<H256> {
        if self.is_authority(&account).await? {
            let signature = account.ecdsa_sign(&message)?;
            match &account.darwinia_account.real {
                // proxy
                Some(real) => {
                    trace!("Proxyed ecdsa sign and submit authorities to darwinia");
                    let submit_signed_authorities = SubmitSignedAuthorities { signature };

                    let ex = self.darwinia.subxt.encode(submit_signed_authorities).unwrap();
                    let tx_hash = self
                        .darwinia
                        .subxt
                        .proxy(
                            &account.darwinia_account.signer,
                            real.clone(),
                            Some(ProxyType::EthereumBridge),
                            &ex,
                        )
                        .await?;
                    Ok(tx_hash)
                }
                None => {
                    trace!("Ecdsa sign and submit authorities to darwinia");
                    let tx_hash = self
                        .darwinia
                        .subxt
                        .submit_signed_authorities(&account.darwinia_account.signer, signature)
                        .await?;
                    Ok(tx_hash)
                }
            }
        } else {
            Err(DarwiniaError::Bridger("Not authority".to_string()).into())
        }
    }

    /// submit_signed_mmr_root
    pub async fn ecdsa_sign_and_submit_signed_mmr_root(
        &self,
        account: &ToEthereumAccount,
        spec_name: String,
        block_number: u32,
    ) -> Result<H256> {
        if self.is_authority(&account).await? {
            // get mmr root from darwinia
            let leaf_index = block_number;
            let mmr_root = self.darwinia.get_mmr_root(leaf_index).await?;

            let encoded = Darwinia2Ethereum::construct_mmr_root_message(spec_name, block_number, mmr_root);
            let hash = web3::signing::keccak256(&encoded);
            let signature = account.ecdsa_sign(&hash)?;

            match &account.darwinia_account.real {
                // proxy
                Some(real) => {
                    trace!(
                        "Proxyed ecdsa sign and submit mmr_root to darwinia, block_number: {}",
                        block_number
                    );
                    let submit_signed_mmr_root = SubmitSignedMmrRoot {
                        block_number,
                        signature,
                    };

                    let ex = self.darwinia.subxt.encode(submit_signed_mmr_root).unwrap();
                    let tx_hash = self
                        .darwinia
                        .subxt
                        .proxy(
                            &account.darwinia_account.signer,
                            real.clone(),
                            Some(ProxyType::EthereumBridge),
                            &ex,
                        )
                        .await?;
                    Ok(tx_hash)
                }
                None => {
                    trace!(
                        "Ecdsa sign and submit mmr_root to darwinia, block_number: {}",
                        block_number
                    );
                    let tx_hash = self
                        .darwinia
                        .subxt
                        .submit_signed_mmr_root(&account.darwinia_account.signer, block_number, signature)
                        .await?;
                    Ok(tx_hash)
                }
            }
        } else {
            Err(DarwiniaError::Bridger("Not authority".to_string()).into())
        }
    }

    /// is authority
    pub async fn is_authority(&self, account: &ToEthereumAccount) -> Result<bool> {
        let authorities = self
            .darwinia
            .subxt
            .authorities(None)
            .await?
            .iter()
            .map(|a| a.account_id.clone())
            .collect::<Vec<_>>();
        Ok(authorities.contains(account.darwinia_account.real()))
    }

    /// need_to_sign_authorities
    pub async fn need_to_sign_authorities(&self, account: &ToEthereumAccount, message: EcdsaMessage) -> Result<bool> {
        let ret = self
            .darwinia
            .subxt
            .authorities_to_sign(None).await?;
        match ret {
            None => Ok(false),
            Some(r) => {
                if r.0 == message {
                    let includes = r.1.iter().any(|a| a.0 == account.darwinia_account.account_id);
                    Ok(!includes)
                } else {
                    Ok(false)
                }
            }
        }
    }

    /// need_to_mmr_root_of
    pub async fn need_to_sign_mmr_root_of(&self, account: &ToEthereumAccount, block_number: u32) -> bool {
        match self
            .darwinia
            .subxt
            .mmr_roots_to_sign(block_number, None)
            .await {
            Ok(mmr_roots_to_sign) => match mmr_roots_to_sign {
                None => false,
                Some(items) => {
                    let includes = items.iter().any(|a| a.0 == account.darwinia_account.account_id);
                    !includes
                }
            },
            Err(err) => {
                error!(
                    "An error was encountered when trying to get storage MMRRootsToSign: {:?}",
                    err
                );
                false
            }
        }
    }

    /// Print Detail
    pub async fn account_detail(&self, account: &ToEthereumAccount) -> Result<()> {
        let mut roles = self.darwinia.account_role(&account.darwinia_account).await?;
        if self.is_authority(&account).await? {
            roles.push("Authority".to_string());
        }
        match &account.darwinia_account.real {
            None => {
                info!("🧔 Relayer({:?}): 0x{:?}", roles, account.darwinia_account.account_id);
            }
            Some(real_account_id) => {
                info!("🧔 Proxy Relayer: 0x{:?}", account.darwinia_account.account_id);
                info!("👴 Real Account({:?}): 0x{:?}", roles, real_account_id);
            }
        }
        Ok(())
    }
}
