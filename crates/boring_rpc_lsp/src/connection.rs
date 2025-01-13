use anyhow::Result;
use lsp_server::{Connection, IoThreads};
use lsp_types::InitializeParams;

use crate::server_capabilities::get_server_capabilities;

/// Create and complete the initialization of connection with LSP client.
pub fn create_connection() -> Result<(Connection, IoThreads)> {
    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = get_server_capabilities();

    let initialization_params: InitializeParams =
        serde_json::from_value(connection.initialize(serde_json::to_value(server_capabilities)?)?)?;

    dbg!(initialization_params);

    Ok((connection, io_threads))
}
