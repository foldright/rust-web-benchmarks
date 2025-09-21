use micro_web::router::{get, Router};
use micro_web::Server;

/// A simple handler that returns "hello world"
async fn hello_world() -> &'static str {
    "hello world"
}

/// Default handler for unmatched handlers (404 responses)
///
/// This handler is called when no other handlers match the incoming request
async fn default_handler() -> &'static str {
    "404 not found"
}

#[tokio::main]
async fn main() {
    // Create a new router using the builder
    let router = Router::builder()
        // Add a route that matches GET requests to the root path "/"
        // converts our async function into a handler
        .route("/", get(hello_world))
        .build();

    // Configure and start the server
    Server::builder()
        // Attach our router to handle incoming requests
        .router(router)
        // Set the address and port to listen on
        .bind("127.0.0.1:3000")
        // Set a handler for requests that don't match any routes
        .default_handler(default_handler)
        // Build the server
        .build()
        .unwrap()
        // Start the server and wait for it to finish
        .start()
        .await;
}