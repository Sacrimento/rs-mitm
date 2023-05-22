use hyper::{Request, Response};
use log::info;
use tower::{Layer, Service};

#[derive(Clone, Copy)]
pub struct Logger<S> {
    inner: S,
}

impl<S> Logger<S> {
    fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for Logger<S>
where
    S: Service<Request<B>> + Clone + Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        info!("{:?} {} {}", req.version(), req.method(), req.uri());
        self.inner.call(req)
    }
}

#[derive(Default)]
pub struct LoggerLayer;

impl LoggerLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for LoggerLayer {
    type Service = Logger<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Logger::new(inner)
    }
}
