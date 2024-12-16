use axum::Router;
use boring_rpc_axum::{BoringRPC, BoringRPCHandler};
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatGetMessageByIdRequest {
    pub id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: f64,
    pub text: String,
}
pub struct Chat<S = ()> {
    inner: BoringRPC<S>,
}
impl<S> Chat<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self { inner: BoringRPC::new() }
    }
    pub fn get_message_by_id<H, T>(self, handler: H) -> Self
    where
        H: BoringRPCHandler<T, ChatGetMessageByIdRequest, Message, S>,
    {
        Self {
            inner: self.inner.handle("/get-message-by-id", handler),
        }
    }
}
impl<S> Into<Router<S>> for Chat<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn into(self) -> Router<S> {
        self.inner.into_router()
    }
}
