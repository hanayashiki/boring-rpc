# Boring RPC LSP Server

## Project Discovery

1. Lazily wait for discovery of `boring_rpc.json` or `.br`
2. For `.br` files, walk up the directory to find the enclosing `boring_rpc.json`.
3. Collect all `boring_rpc.json`
4. Group them as different projects.


5. Check each project with `boring_rpc_analyzer`.

## Reference
1. LSP Spec: 
    1. https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/
    2. Watch Files:
        1. https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_didChangeWatchedFiles
    3. Semantic Tokens:
        1. https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_semanticTokens
    4. Syntax Highlighting:
        1. https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide
        2. Treating all as sementic tokens does work.
        3. RA: https://rust-analyzer.github.io/manual.html#semantic-syntax-highlighting
        4. VSCode supports Rust highlighting natively:
            1. Link: https://github.com/microsoft/vscode/tree/964233731a98e6c92ccf377b0415a746e9695b2e/extensions/rust
            2. So the highlighting before the server is up and running is provided from the built-in extension, and later filled with RA's semantic highlighting.
        5. Writing a textmate syntax is painful and for now semantic highlighting from the AST needs to be written.
    5. Sync documents
        1. https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_synchronization
        2. RA handles notification: crates/rust-analyzer/src/handlers/notification.rs
        3. textDocument/didChange vs didSave
            1. RA:
                1. Handles didChange with set_file_contents (modify its vfs)
                2. Handles didSave without modifying the vfs directly, but checks if workspace has changed, and triggers flycheck 
            2. I think didSave means saving to disk. Basically, we only need to handle didChange, because we don't really care about what the disk for the analyzer. But RA handles much diagnostics to rustc.
1. Gleam:
    1. Client: https://github.com/gleam-lang/vscode-gleam/blob/main/src/extension.ts
    2. Server: https://github.com/gleam-lang/gleam/blob/bc628541b3b23d06b3729340b78a0dc5624d0e5c/compiler-cli/src/lsp.rs
1. LSP Problems:
    1. https://www.michaelpj.com/blog/2024/09/03/lsp-good-bad-ugly.html
    2. https://matklad.github.io/2023/10/12/lsp-could-have-been-better.html