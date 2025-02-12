use axum::{
    response::Response,
    body::Body,
    extract::Request,
};
use futures_util::future::BoxFuture;
use tower::{Service, Layer};
use std::task::{Context, Poll};

#[derive(Clone)]
struct {{layer_name|pascalcase}};

impl<S> Layer<S> for {{layer_name|pascalcase}} {
    type Service = {{middleware_name|pascalcase}}<S>;

    fn layer(&self, inner: S) -> Self::Service {
        {{middleware_name|pascalcase}} { inner }
    }
}

#[derive(Clone)]
struct {{middleware_name|pascalcase}}<S> {
    inner: S,
}

impl<S> Service<Request> for {{middleware_name|pascalcase}}<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        unimplemented!()
    }
}
