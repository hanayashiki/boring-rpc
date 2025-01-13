use std::panic::UnwindSafe;

use crate::LspState;

use anyhow::Result;
use lsp_types::request::Request;

/// Handles the request with corresponding handler synchronously.
pub(crate) struct RequestDispatcher<'a> {
    pub(crate) request: lsp_server::Request,
    pub(crate) lsp_state: &'a mut LspState,
}

impl RequestDispatcher<'_> {
    pub fn on_mut<R>(&mut self, f: fn(R::Params) -> Result<R::Result>) -> &mut Self
    where
        R: Request,
        R::Params: UnwindSafe,
    {
        unimplemented!()
    }
}
