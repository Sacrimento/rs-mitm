use env_logger::Env;
use hyper::{service::make_service_fn, Body, Client, Request, Response, Server};
use log::info;
use std::sync::Arc;
use std::{convert::Infallible, net::SocketAddr};
use tower::ServiceBuilder;

pub mod proxy;
use proxy::config::Config;
use proxy::services::{http_dump::layer::HttpDumpLayer, logging::layer::LoggerLayer};

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Client::new().request(req).await
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Arc::new(Config::new().expect("Invalid config"));

    let make_service = make_service_fn(move |_| async move {
        Ok::<_, Infallible>(
            ServiceBuilder::new()
                .layer(LoggerLayer::new())
                .layer(HttpDumpLayer::new())
                .service_fn(handle),
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
