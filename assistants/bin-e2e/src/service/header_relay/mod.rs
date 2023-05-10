pub mod beacon_header_relay;
pub mod sync_committee_relay;

pub mod types {
    use bridge_e2e_traits::client::OnDemandHeader;

    #[derive(Debug, Clone)]
    pub struct DarwiniaHeader(pub u64);

    impl From<u64> for DarwiniaHeader {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl OnDemandHeader for DarwiniaHeader {
        fn block_number(&self) -> u64 {
            self.0
        }
    }

    #[derive(Debug, Clone)]
    pub struct EthereumHeader(pub u64);
    impl From<u64> for EthereumHeader {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }

    impl OnDemandHeader for EthereumHeader {
        fn block_number(&self) -> u64 {
            self.0
        }
    }
}
