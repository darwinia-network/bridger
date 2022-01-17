use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use support_terminal::output::OutputFormat;

/// Namespace kv options, the special namespace is allowed
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub struct NamespaceKvOpts {
    /// Namespace
    #[structopt(short, long)]
    pub namespace: Option<String>,
    /// Kv commands
    #[structopt(flatten)]
    pub command: KvOpts,
}

/// Kv options
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub enum KvOpts {
    /// Show all namespaces
    Namespaces,
    /// Put Key-Value to bridger database
    Put {
        /// Keys and Values one by one
        #[structopt()]
        kvs: Vec<String>,
    },
    /// Get Key-Value from bridger
    Get {
        /// Get a value by key
        #[structopt()]
        keys: Vec<String>,
        /// Output mode, support  raw|table|json
        #[structopt(short, long, default_value = "raw")]
        output: OutputFormat,
        /// The output is include key
        #[structopt(long)]
        include_key: bool,
    },
    /// List bridger database
    Keys {
        /// List by sorted
        #[structopt(short, long)]
        sorted: bool,
    },
    /// Remove a Key-Value from bridger
    Remove {
        /// Remove a value by key
        #[structopt()]
        keys: Vec<String>,
    },
}
