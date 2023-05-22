use std::{net::SocketAddr};
use tower::make::Shared;
use hyper::{service::{service_fn}, Server, Request, Body, Response, Client};

pub mod proxy;
use proxy::config::Settings;

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("{:?}", req);
    let client = Client::new();
    client.request(req).await
}

#[tokio::main]
async fn main() {
    let config = Settings::new().expect("Invalid config");
    println!("{:#?}", config);

    let make_service = Shared::new(service_fn(handle));

    let addr = SocketAddr::from((config.proxy.host_as_ip(), config.proxy.port));
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        panic!("Error initializing the HTTP server: {}", e);
    }
}
