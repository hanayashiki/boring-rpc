use axum::{
    extract::{FromRequest, FromRequestParts, Json, Request, State},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, pin::Pin};

pub struct BoringRPC<S = ()> {
    router: Router<S>,
}

impl<S> BoringRPC<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    pub fn handle<T, Req, Res, H>(self, method: &str, handler: H) -> Self
    where
        Req: DeserializeOwned,
        Res: Serialize,
        H: BoringRPCHandler<T, Req, Res, S>,
    {
        Self {
            router: self.router.route(
                method,
                post(|State(state): State<S>, req: Request| handler.call(req, state)),
            ),
        }
    }

    pub fn into_router(self) -> Router<S> {
        self.router
    }

    pub fn with_state<S2>(self, state: S) -> BoringRPC<S2> {
        BoringRPC {
            router: self.router.with_state(state),
        }
    }
}

pub trait BoringRPCHandler<T, Req, Res, S>: Clone + Send + Sync + Sized + 'static {
    type Future: Future<Output = Response> + Send + 'static;

    /// Call the handler with the given request.
    fn call(self, req: Request, state: S) -> Self::Future;
}

macro_rules! impl_rpc_handler {
    (
        $($ty:ident),*
    ) => {
        impl<F, Fut, S, Req, Res, $($ty,)*> BoringRPCHandler<($($ty,)*), Req, Res, S> for F
        where
            F: FnOnce($($ty,)* Req) -> Fut + Clone + Send + Sync + 'static,
            Fut: Future<Output = Res> + Send,
            S: Send + Sync + 'static,
            Req: DeserializeOwned + Send,
            Res: Serialize,
            $( $ty: FromRequestParts<S> + Send, )*
        {
            type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

            fn call(self, req: Request, state: S) -> Self::Future {
                Box::pin(async move {
                    let (mut parts, body) = req.into_parts();
                    let state = &state;

                    $(
                        let $ty = match $ty::from_request_parts(&mut parts, state).await {
                            Ok(value) => value,
                            Err(rejection) => return rejection.into_response(),
                        };
                    )*

                    let req = Request::from_parts(parts, body);

                    let rpc_req = match Json::<Req>::from_request(req, state).await {
                        Ok(value) => value,
                        Err(rejection) => return rejection.into_response(),
                    };

                    let res = self($($ty,)* rpc_req.0,).await;

                    Json(res).into_response()
                })
            }
        }
    }
}

all_the_tuples!(impl_rpc_handler);
