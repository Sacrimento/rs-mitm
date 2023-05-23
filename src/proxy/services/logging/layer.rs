use tower::Layer;

use super::service::LoggerService;

#[derive(Default)]
pub struct LoggerLayer;

impl LoggerLayer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for LoggerLayer {
    type Service = LoggerService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggerService::new(inner)
    }
}
