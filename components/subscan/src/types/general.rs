use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtrinsicsData {
    pub count: u32,
    pub extrinsics: Vec<Extrinsic>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Extrinsic {
    /*
    https://docs.api.subscan.io/#extrinsic
    {
        "account_display": {
            "account_index": "",
            "address": "16hp43x8DUZtU8L3cJy9Z8JMwTzuu8ZZRWqDZnpMhp464oEd",
            "display": "",
            "identity": false,
            "judgements": null,
            "parent": "",
            "parent_display": ""
        },
        "account_id": "16hp43x8DUZtU8L3cJy9Z8JMwTzuu8ZZRWqDZnpMhp464oEd",
        "block_num": 2028659,
        "block_timestamp": 1602732510,
        "call_module": "balances",
        "call_module_function": "transfer",
        "error": null,
        "event": [
            {
                "block_num": 2028659,
                "event_id": "Transfer",
                "event_idx": 2,
                "event_index": "2028659-2",
                "extrinsic_hash": "0x3d944ee0f1aead94a50730eec73aba16c930ca9f468a63d11415c73c80e325ba",
                "extrinsic_idx": 2,
                "finalized": true,
                "module_id": "balances",
                "params": "[{\"type\":\"AccountId\",\"value\":\"fc4d6069cb980fcde6ef73a46f1894d66c152e9ddfac3a499cf6a1654ff5f55b\",\"value_raw\":\"\"},{\"type\":\"AccountId\",\"value\":\"105fdf37958064a0c87096bd9cfbd307f955f275f3979c40fa701118ec0e657a\",\"value_raw\":\"\"},{\"type\":\"Balance\",\"value\":\"2561090000000\",\"value_raw\":\"\"}]"
            }
        ],
        "event_count": 3,
        "extrinsic_hash": "0x3d944ee0f1aead94a50730eec73aba16c930ca9f468a63d11415c73c80e325ba",
        "extrinsic_index": "2028659-2",
        "fee": "156000000",
        "finalized": true,
        "lifetime": {
            "birth": 2028654,
            "death": 2029166
        },
        "nonce": 5786,
        "params": [
            {
                "name": "dest",
                "type": "Address",
                "value": "105fdf37958064a0c87096bd9cfbd307f955f275f3979c40fa701118ec0e657a",
                "valueRaw": ""
            },
            {
                "name": "value",
                "type": "Compact<Balance>",
                "value": "2561090000000",
                "valueRaw": ""
            }
        ],
        "signature": "a672d84b55afd71df5fd45e48043b479e762124ff174db116ba5cd979040544b3b536c2e66a72231a698d4d0588f89470e25acbe8ff39ba7b78d302034a63e00",
        "success": true,
        "tip": "0",
        "transfer": {
            "amount": "256.109",
            "block_num": 0,
            "block_timestamp": 0,
            "extrinsic_index": "",
            "fee": "0",
            "from": "16hp43x8DUZtU8L3cJy9Z8JMwTzuu8ZZRWqDZnpMhp464oEd",
            "hash": "0x3d944ee0f1aead94a50730eec73aba16c930ca9f468a63d11415c73c80e325ba",
            "module": "balances",
            "success": true,
            "to": "1NUFGnNcUpGh2biqtQ1yND7NCDY3q2Eoh7PbsurheeSNmAH",
            "to_account_display": {
                "account_index": "",
                "address": "1NUFGnNcUpGh2biqtQ1yND7NCDY3q2Eoh7PbsurheeSNmAH",
                "display": "",
                "identity": false,
                "judgements": null,
                "parent": "",
                "parent_display": ""
            }
        }
    },
     */
    // pub account_display: null,
    #[serde(with = "SerHex::<StrictPfx>")]
    pub account_id: [u8; 32],
    // pub account_index: "",
    pub block_num: u32,
    pub block_timestamp: u64,
    pub call_module: String,
    pub call_module_function: String,
    #[serde(with = "SerHex::<StrictPfx>")]
    pub extrinsic_hash: [u8; 32],
    pub extrinsic_index: String,
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub fee: u128,
    pub nonce: u64,
    #[serde(deserialize_with = "super::patch::smart_deserialize_param")]
    pub params: Vec<Param>,
    #[serde(with = "SerHex::<StrictPfx>")]
    pub signature: [u8],
    pub success: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountDisplay {
    // pub account_index: "",
    #[serde(with = "SerHex::<StrictPfx>")]
    pub address: [u8; 32],
    pub display: String,
    pub identity: bool,
    // pub judgements: null,
    #[serde(with = "SerHex::<StrictPfx>")]
    pub parent: [u8; 32],
    pub parent_display: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(with = "SerHex::<StrictPfx>")]
    pub value: [u8],
    // pub value_raw: ""
}
