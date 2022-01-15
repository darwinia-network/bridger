use structopt::StructOpt;

use support_command_kv::NamespaceKvOpts;
use support_terminal::types::BasicOptions;

use crate::command::types::{
    AffirmOpts, ConfirmOpts, EcdsaOpts, InfoOpts, MmrOpts, ParcelOpts, RelayOpts,
};

/// Bridge pangolin-ropsten options
#[derive(Debug, StructOpt)]
#[structopt(name = "pangolin-ropsten", about = "Bridge pangolin-ropsten")]
pub enum Opts {
    /// Start bridge pangolin-ropsten
    Start,
    /// Do affirm
    Affirm {
        /// Commands of affirm
        #[structopt(flatten)]
        command: AffirmOpts,
    },
    /// Do confirm
    Confirm {
        /// Commands of confirm
        #[structopt(flatten)]
        command: ConfirmOpts,
    },
    /// Ecdsa
    Ecdsa {
        /// Commands of ecdsa
        #[structopt(flatten)]
        command: EcdsaOpts,
    },
    /// Guard
    Guard,
    /// Info
    Info {
        /// Commands of info
        #[structopt(flatten)]
        command: InfoOpts,
    },
    /// Keys
    Keys,
    /// MMR
    Mmr {
        /// Commands of mmr
        #[structopt(flatten)]
        command: MmrOpts,
    },
    /// Parcel
    Parcel {
        /// Commands of parcel
        #[structopt(flatten)]
        command: ParcelOpts,
        /// Basic options
        #[structopt(flatten)]
        output: BasicOptions,
    },
    /// Relay
    Relay {
        /// Commands of parcel
        #[structopt(flatten)]
        command: RelayOpts,
    },
    /// Kv command
    Kv {
        /// Commands of kv
        #[structopt(flatten)]
        command: NamespaceKvOpts,
    },
}
