//! Custom byte array
use codec::{Decode, Encode};
use serde::{
    de::{Deserialize, Deserializer, Error, SeqAccess, Visitor},
    ser::{Serialize, SerializeTuple, Serializer},
};
use std::{
    fmt::{self, Debug, Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};
use uint::construct_uint;

/// Big Array Serde
pub trait BigArray<'de>: Sized {
    /// Serialize big array
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
    /// Deserialize big array
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

// shared
construct_uint! {
    #[derive(Encode, Decode, Serialize, Deserialize)]
    pub struct U256(4);
}

construct_hash_bytes! {
    #[derive(Clone)]
    pub struct H128(16);
}

construct_hash_bytes! {
    #[derive(Clone)]
    pub struct H512(64);
}

construct_hash_bytes! {
    #[derive(Clone)]
    pub struct H1024(256);
}
