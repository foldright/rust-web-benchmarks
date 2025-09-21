#![deny(warnings)]

use std::net::SocketAddr;
use tokio::net::TcpListener;
use viz::{get, Request, Result, Router, serve};

async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello, World!")
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    let app = Router::new().route("/", get(index));

    if let Err(e) = serve(listener, app).await {
        println!("{e}");
    }

    Ok(())
}
