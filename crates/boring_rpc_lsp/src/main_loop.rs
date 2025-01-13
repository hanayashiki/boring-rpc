use anyhow::Result;
use lsp_server::{Connection, Message};
use lsp_types::notification::{self, DidChangeTextDocument};

use crate::{LspState, NotificationDispatcher, RequestDispatcher};

pub fn main_loop(connection: Connection) -> Result<()> {
    let mut lsp_state = LspState::default();

    for message in &connection.receiver {
        match message {
            Message::Request(request) => {
                let dispatcher = RequestDispatcher {
                    request,
                    lsp_state: &mut lsp_state,
                };
            }
            Message::Notification(notification) => {
                NotificationDispatcher::new(notification, &mut lsp_state)
                    .on_mut::<DidChangeTextDocument>(LspState::handle_did_change_text_document);
            }
            _ => {
                eprintln!("message not handled: {:?}", message);
            }
        }
    }

    Ok(())
}
