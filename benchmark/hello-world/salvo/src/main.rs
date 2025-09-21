use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {

    let acceptor = TcpListener::new("127.0.0.1:3000").bind().await;

    let router = Router::new().get(hello);

    Server::new(acceptor)
        .serve(router)
        .await
}
