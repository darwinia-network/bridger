//! Custom byte array

use std::{
    fmt::{self, Debug, Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};

use codec::{Decode, Encode};
use serde::{
    de::{Deserialize, Deserializer, Error, SeqAccess, Visitor},
    ser::{Serialize, SerializeTuple, Serializer},
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

// todo: may there can direct use sp_core::primitive_types?

// shared
construct_uint! {
    #[derive(Encode, Decode, Serialize, Deserialize)]
    pub struct U256(4);
}

crate::construct_hash_bytes! {
    #[derive(Clone)]
    pub struct H128(16);
}

crate::construct_hash_bytes! {
    #[derive(Clone)]
    pub struct H160(20);
}

crate::construct_hash_bytes! {
    #[derive(Clone)]
    pub struct H256(32);
}

crate::construct_hash_bytes! {
    #[derive(Clone)]
    pub struct H512(64);
}

crate::construct_hash_bytes! {
    #[derive(Clone)]
    pub struct H1024(128);
}

crate::construct_hash_bytes! {
    #[derive(Clone)]
    pub struct Bloom(256);
}

crate::impl_hash_rlp!(Bloom, 256);
crate::impl_hash_rlp!(H256, 32);
crate::impl_hash_rlp!(H160, 20);
