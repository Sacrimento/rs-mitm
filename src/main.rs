use std::{net::SocketAddr};
use tower::make::Shared;
use hyper::{service::{service_fn}, Server, Request, Body, Response, Client};

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("{:?}", req);
    let client = Client::new();
    client.request(req).await
}

#[tokio::main]
async fn main() {
    let make_service = Shared::new(service_fn(handle));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        panic!("Error initializing the HTTP server: {}", e);
    }
}
