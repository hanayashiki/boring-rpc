/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import type { ExtensionContext } from "vscode";

import type {
  LanguageClientOptions,
  Executable,
} from "vscode-languageclient/node";
import { LanguageClient, TransportKind } from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(_context: ExtensionContext) {
  // FIXME: no hard coding
  const serverOptions: Executable = {
    command: "/Users/chenyuwang/boring-rpc/target/debug/boring_rpc_lsp",
    transport: TransportKind.stdio,
    options: {
      env: {
        RUST_BACKTRACE: "1",
      }
    }
  };

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    // Register the server for plain text documents
    documentSelector: [{ scheme: "file", language: "boring-rpc" }],
  };

  // Create the language client and start the client.
  client = new LanguageClient(
    "BoringRPC",
    "Boring RPC",
    serverOptions,
    clientOptions,
  );

  // Start the client. This will also launch the server
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
