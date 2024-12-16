use axum::Router;
use boring_rpc_axum::{BoringRPC, BoringRPCHandler};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Method1Request {
    pub question: String,
}
#[derive(Serialize)]
pub struct Method1Response {
    pub answer: bool,
}

#[derive(Deserialize)]
pub struct Method2Request {}
#[derive(Serialize)]
pub struct Method2Response {}

pub struct ExampleService<S = ()> {
    inner: BoringRPC<S>,
}

impl<S> ExampleService<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            inner: BoringRPC::new(),
        }
    }

    pub fn method1<H, T>(self, handler: H) -> Self
    where
        H: BoringRPCHandler<T, Method1Request, Method1Response, S>,
    {
        Self {
            inner: self.inner.handle("/method1", handler),
        }
    }

    pub fn method2<H, T>(self, handler: H) -> Self
    where
        H: BoringRPCHandler<T, Method2Request, Method2Response, S>,
    {
        Self {
            inner: self.inner.handle("/method2", handler),
        }
    }

    pub fn with_state<S2>(self, state: S) -> ExampleService<S2> {
        ExampleService {
            inner: self.inner.with_state(state),
        }
    }
}

impl<S> Into<Router<S>> for ExampleService<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn into(self) -> Router<S> {
        self.inner.into_router()
    }
}
