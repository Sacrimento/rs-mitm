use hyper::{
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};
use std::{convert::Infallible, net::SocketAddr};
use tower::ServiceBuilder;

pub mod proxy;
use proxy::config::Settings;
use proxy::log_layer::LoggerLayer;

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Client::new().request(req).await
}

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let config = Settings::new().expect("Invalid config");

    let make_service = make_service_fn(|_| async {
        Ok::<_, Infallible>(
            ServiceBuilder::new()
                .layer(LoggerLayer::new())
                .service(service_fn(handle)),
        )
    });

    let addr = SocketAddr::from((config.proxy.host, config.proxy.port));

    Server::bind(&addr).serve(make_service).await?;

    Ok(())
}
