use http;
use hyper::{Request, Response};
use log::info;
use pin_project::pin_project;
use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::Instant;
use tower::Service;

#[derive(Clone, Copy)]
pub struct LoggerService<S> {
    inner: S,
}

#[pin_project]
pub struct ResponseFuture<T> {
    #[pin]
    inner: T,
    req_context: LogContext,
}

struct LogContext {
    method: http::Method,
    authority: http::uri::Authority,
    path: http::uri::PathAndQuery,
    version: http::Version,
    start: Instant,
}

impl<S> LoggerService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, B> Service<Request<B>> for LoggerService<S>
where
    S: Service<Request<B>, Response = Response<B>> + Clone + Send,
    S::Error: fmt::Display + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let ctx = LogContext {
            method: req.method().clone(),
            authority: req.uri().authority().unwrap().clone(),
            path: req.uri().path_and_query().unwrap().clone(),
            version: req.version(),
            start: Instant::now(),
        };

        let f = self.inner.call(req);

        ResponseFuture {
            inner: f,
            req_context: ctx,
        }
    }
}

impl<T, B, E> Future for ResponseFuture<T>
where
    T: Future<Output = Result<Response<B>, E>>,
    E: std::fmt::Display + 'static,
{
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let result = this.inner.poll(cx);

        match result {
            Poll::Ready(Ok(res)) => {
                let req = this.req_context;

                info!(
                    "{:?} {} {} {} - {} in {} ms",
                    req.version,
                    req.method,
                    req.authority,
                    req.path,
                    res.status(),
                    req.start.elapsed().as_millis(),
                );

                return Poll::Ready(Ok(res));
            }
            Poll::Ready(Err(err)) => return Poll::Ready(Err(err)),
            Poll::Pending => {}
        }

        result
    }
}
