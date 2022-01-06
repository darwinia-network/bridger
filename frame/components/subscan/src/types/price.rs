use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenPrice {
    /*
    {
        "code": 0,
        "message": "Success",
        "generated_at": 1593479990,
        "data": {
            "price": "5.9032161816",
            "time": 1593391878,
            "height": 479676,
            "records": [
                {
                    "price": "5.9032161816",
                    "height": 479676,
                    "time": 1593391878
                }
            ]
        }
    }
     */
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub price: f64,
    pub time: u64,
    pub height: u32,
    pub records: Vec<OpenPriceRecord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenPriceRecord {
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    pub price: f64,
    pub time: u64,
    pub height: u32,
}
