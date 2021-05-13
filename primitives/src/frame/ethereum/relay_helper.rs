use substrate_subxt::Runtime;
use crate::frame::ethereum::relay::EthereumRelay;

pub trait EthereumRelayHelper: Runtime + EthereumRelay {
    fn get_pending_relay_header_number(parcel: Self::PendingRelayHeaderParcel)
        -> Self::EthereumBlockNumber;
}
