use gql_client::Client;
use include_dir::{include_dir, Dir};

use crate::error::{TheGraphLikethComponentError, ThegraphComponentReuslt};
use crate::types::LikethChain;

#[cfg(feature = "bridge-ethv2")]
mod bridge_ethv2;

/// Graphql dir
static GRAPHQL_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/graphql");

/// thegraph toolkit, query ethereum/ropsten
#[derive(Clone, Debug)]
pub struct Thegraph {
    client: Client,
    chain: LikethChain,
}

impl Thegraph {
    pub fn new(client: Client, chain: LikethChain) -> Self {
        Self { client, chain }
    }
}

impl Thegraph {
    fn read_graphql(&self, file: impl AsRef<str>) -> ThegraphComponentReuslt<&str> {
        let file = file.as_ref();
        let dir = self.chain.directory();
        let graph = GRAPHQL_DIR
            .get_file(format!("{}/{}", dir, file))
            .or_else(|| GRAPHQL_DIR.get_file(format!("generic/{}", file)))
            .ok_or_else(|| {
                TheGraphLikethComponentError::GraphQL("No graphql fround".to_string())
            })?;
        graph.contents_utf8().ok_or_else(|| {
            TheGraphLikethComponentError::GraphQL("Failed to read graphql file".to_string())
        })
    }
}
