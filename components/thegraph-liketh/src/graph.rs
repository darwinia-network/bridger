use gql_client::Client;

pub struct TheGraphLikeEth<'a> {
    client: Client<'a>,
}

impl<'a> TheGraphLikeEth<'a> {
    pub fn new(client: Client<'a>) -> Self {
        Self { client }
    }
}
