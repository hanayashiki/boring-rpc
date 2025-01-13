use anyhow::Result;
use boring_rpc_lsp::{create_connection, main_loop};

fn main() -> Result<()> {
    let (connection, io_threads) = create_connection()?;

    main_loop(connection)?;
 
    io_threads.join()?;
    Ok(())
}
