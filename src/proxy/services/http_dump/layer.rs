use tower::Layer;

use super::service::HttpDumpService;

#[derive(Default)]
pub struct HttpDumpLayer;

impl HttpDumpLayer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for HttpDumpLayer {
    type Service = HttpDumpService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HttpDumpService::new(inner)
    }
}
