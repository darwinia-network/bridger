use tide::prelude::*;
use tide::Request;

use crate::types::command::ServerOptions;

pub async fn handle_server(options: ServerOptions) -> anyhow::Result<()> {
    let mut app = tide::new();

    app.at("/orders/shoes").post(order_shoes);

    let addr = format!("{}:{}", options.host, options.port);
    println!("Bridger service listen: {}", addr);
    app.listen(&addr).await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
