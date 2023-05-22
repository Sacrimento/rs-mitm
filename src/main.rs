use env_logger;
use env_logger::Env;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};
use log::info;
use std::{convert::Infallible, net::SocketAddr};
use tower::ServiceBuilder;

pub mod proxy;
use proxy::config::Config;
use proxy::log_layer::LoggerLayer;

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Client::new().request(req).await
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::new().expect("Invalid config");

    let make_service = make_service_fn(|_| async {
        Ok::<_, Infallible>(
            ServiceBuilder::new()
                .layer(LoggerLayer::new())
                .service(service_fn(handle)),
        )
    });

    let addr = SocketAddr::from((config.proxy.host, config.proxy.port));

    info!(
        "Starting MITM proxy service on {}:{}",
        config.proxy.host, config.proxy.port
    );

    Server::bind(&addr).serve(make_service).await?;

    Ok(())
}
