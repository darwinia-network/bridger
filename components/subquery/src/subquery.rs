use gql_client::Client;

#[derive(Clone, Debug, Default)]
pub struct Subquery {
    client: Client,
}

impl Subquery {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl Subquery {}
