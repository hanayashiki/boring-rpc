use std;

use anyhow::Result;
use boring_rpc_vfs::LspFs;
use lsp_types::DidChangeTextDocumentParams;

#[derive(Default, Debug)]
pub(crate) struct LspState {
    fs: LspFs,
}

impl LspState {
    pub fn handle_did_change_text_document(
        &mut self,
        notif: DidChangeTextDocumentParams,
    ) -> Result<()> {
        dbg!(notif);
        // self.fs.set_file_content(path, change);

        Ok(())
    }
}
