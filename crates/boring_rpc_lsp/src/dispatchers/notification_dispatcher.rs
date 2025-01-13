use std::panic::UnwindSafe;

use crate::LspState;

use anyhow::Result;
use lsp_types::notification::Notification;

/// Handles the request with corresponding handler synchronously.
pub(crate) struct NotificationDispatcher<'a> {
    pub(crate) notification: Option<lsp_server::Notification>,
    pub(crate) lsp_state: &'a mut LspState,
}

impl<'a> NotificationDispatcher<'a> {
    pub fn new(notification: lsp_server::Notification, lsp_state: &'a mut LspState) -> Self {
        Self {
            notification: Some(notification),
            lsp_state,
        }
    }

    pub fn on_mut<R>(
        &mut self,
        f: fn(lsp_state: &mut LspState, R::Params) -> Result<()>,
    ) -> &mut Self
    where
        R: Notification,
        R::Params: UnwindSafe,
    {
        match self.parse::<R>() {
            Some(params) => f(self.lsp_state, params),
            _ => Ok(())
        }.expect("TODO: handle error");

        self
    }

    /// Consume `.notification`, returning the parsed params of the notification if matching.
    pub fn parse<R>(&mut self) -> Option<R::Params>
    where
        R: Notification,
        R::Params: UnwindSafe,
    {
        match self.notification.take_if(|n| n.method == R::METHOD) {
            Some(lsp_server::Notification { params, .. }) => {
                serde_json::from_value(params).expect("Valid request")
            }
            _ => None,
        }
    }
}
