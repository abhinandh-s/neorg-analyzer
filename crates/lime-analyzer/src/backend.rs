#![allow(dead_code)]

use std::sync::Arc;

use dashmap::DashMap;
use ropey::Rope;
use serde_json::Value;
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};

use crate::span::position_to_offset;

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    // Maps a document URI to its text content
    pub doc_map: DashMap<Arc<String>, Rope>,
}

impl Backend {
    /// params will give the range of the document that changed and the actual content changes
    /// we will we will replace that range in doc_map with changed content
    /// -- FIX: not tested
    async fn on_change(&self, params: DidChangeTextDocumentParams) {
        // The document that did change.
        let uri = params.text_document.uri.to_string();
        for change in params.content_changes {
            // The range of the document that changed.
            if let Some(range) = change.range {
                // The actual content changes.
                let text = change.text;
                let (start, end) = (range.start, range.end);
                if let Some(doc) = self.doc_map.get(&uri.to_string()) {
                    let mut rope = doc.value().to_owned();
                    let start_idx = position_to_offset(start, &rope).get_or_insert(0).to_owned();
                    let end_idx = position_to_offset(end, &rope).get_or_insert(0).to_owned();
                    rope.remove(start_idx..end_idx);
                    rope.insert(start_idx, &text);
                    self.doc_map.insert(uri.clone().into(), rope);
                }
            }
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_owned()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_owned()],
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        let _p = params.event.added;
        self.client
            .log_message(MessageType::INFO, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "watched files have changed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text_document = params.text_document.clone();
        self.doc_map.insert(
            text_document.uri.to_string().into(),
            Rope::from_str(text_document.text.to_string().as_str()),
        );
        self.client
            .log_message(MessageType::ERROR, "file opened!")
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.on_change(params).await;
        self.client
            .log_message(MessageType::ERROR, "file changed!")
            .await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(provide_completions())
    }
}

pub fn provide_completions() -> Option<CompletionResponse> {
    Some(CompletionResponse::Array(vec![CompletionItem::new_simple(
        "Bye".to_owned(),
        "More detail".to_owned(),
    )]))
}
