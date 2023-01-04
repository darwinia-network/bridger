impl From<crate::crypto::ethereum::Signature> for sp_core::ecdsa::Signature {
    fn from(origin: crate::crypto::ethereum::Signature) -> Self {
        sp_core::ecdsa::Signature(origin.0)
    }
}

impl From<crate::crypto::ethereum::Public> for sp_core::ecdsa::Public {
    fn from(origin: crate::crypto::ethereum::Public) -> Self {
        sp_core::ecdsa::Public(origin.0)
    }
}

impl From<crate::crypto::ethereum::Signature> for account::EthereumSignature {
    fn from(signature: crate::crypto::ethereum::Signature) -> Self {
        sp_core::ecdsa::Signature(signature.0).into()
    }
}

impl From<crate::crypto::ethereum::Public> for account::EthereumSigner {
    fn from(public: crate::crypto::ethereum::Public) -> Self {
        sp_core::ecdsa::Public(public.0).into()
    }
}
