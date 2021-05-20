/// Create chain-specific set of configuration objects: connection parameters,
/// signing parameters and bridge initialisation parameters.
#[macro_export]
macro_rules! declare_chain_options {
	($chain:ident, $chain_prefix:ident) => {
		paste::item! {
			#[doc = $chain " connection params."]
			#[derive(StructOpt, Debug, Clone, PartialEq, Eq)]
			pub struct [<$chain ConnectionParams>] {
				#[doc = "Connect to " $chain " node at given host."]
				#[structopt(long, default_value = "127.0.0.1")]
				pub [<$chain_prefix _host>]: String,
				#[doc = "Connect to " $chain " node websocket server at given port."]
				#[structopt(long)]
				pub [<$chain_prefix _port>]: u16,
				#[doc = "Use secure websocket connection."]
				#[structopt(long)]
				pub [<$chain_prefix _secure>]: bool,
			}

			#[doc = $chain " signing params."]
			#[derive(StructOpt, Debug, Clone, PartialEq, Eq)]
			pub struct [<$chain SigningParams>] {
				#[doc = "The SURI of secret key to use when transactions are submitted to the " $chain " node."]
				#[structopt(long)]
				pub [<$chain_prefix _signer>]: String,
				#[doc = "The password for the SURI of secret key to use when transactions are submitted to the " $chain " node."]
				#[structopt(long)]
				pub [<$chain_prefix _signer_password>]: Option<String>,
			}

		}
	};
}
