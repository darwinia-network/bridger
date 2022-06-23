use core::fmt::Debug;

use codec::{Codec, Encode, EncodeLike};
use num_traits::{Bounded, CheckedSub, SaturatingAdd, Zero};
use sp_runtime::{
    traits::{
        AtLeast32Bit, AtLeast32BitUnsigned, Extrinsic, Hash, Header, MaybeSerializeDeserialize,
        Member, Verify,
    },
    FixedPointOperand,
};

/// Runtime types.
pub trait Config: 'static {
    /// Account index (aka nonce) type. This stores the number of previous
    /// transactions associated with a sender account.
    type Index: Parameter + Member + Default + AtLeast32Bit + Copy + scale_info::TypeInfo;

    /// The block number type used by the runtime.
    type BlockNumber: Parameter + Member + Default + Copy + core::hash::Hash + core::str::FromStr;

    /// The output of the `Hashing` function.
    type Hash: Parameter
        + Member
        + MaybeSerializeDeserialize
        + Ord
        + Default
        + Copy
        + std::hash::Hash
        + AsRef<[u8]>
        + AsMut<[u8]>
        + scale_info::TypeInfo;

    type Balance: AtLeast32BitUnsigned
        + FixedPointOperand
        + Parameter
        + Member
        + MaybeSerializeDeserialize
        + Clone
        + Copy
        + Bounded
        + CheckedSub
        + PartialOrd
        + SaturatingAdd
        + Zero
        + TryFrom<sp_core::U256>;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    type Hashing: Hash<Output = Self::Hash>;

    /// The user account identifier type for the runtime.
    type AccountId: Parameter + Member;

    /// The address type. This instead of `<frame_system::Trait::Lookup as StaticLookup>::Source`.
    type Address: Codec + Clone + PartialEq;

    /// The block header.
    type Header: Parameter
        + Header<Number = Self::BlockNumber, Hash = Self::Hash>
        + serde::de::DeserializeOwned;

    /// Signature type.
    type Signature: Verify + Encode + Send + Sync + 'static;

    /// Extrinsic type within blocks.
    type Extrinsic: Parameter + Extrinsic + Debug + MaybeSerializeDeserialize;
}

/// Parameter trait copied from `substrate::frame_support`
pub trait Parameter: Codec + EncodeLike + Clone + Eq + Debug {}

impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + Debug {}
