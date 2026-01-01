use tower_lsp::lsp_types::Position;

use dashmap::DashMap;

use ropey::Rope;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::notification::Notification;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct Backend {
    pub client: Client,
    pub document_map: DashMap<String, Rope>,
    pub cst_map: DashMap<String, neorg_syntax::SyntaxNode>,
}

/*




    /// The [`textDocument/prepareRename`] request is sent from the client to the server to setup
    /// and test the validity of a rename operation at a given location.
    ///
    /// [`textDocument/prepareRename`]: https://microsoft.github.io/language-server-protocol/specification#textDocument_prepareRename
    ///
    /// # Compatibility
    ///
    /// This request was introduced in specification version 3.12.0.
    #[rpc(name = "textDocument/prepareRename")]
    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        let _ = params;
        error!("Got a textDocument/prepareRename request, but it is not implemented");
        Err(Error::method_not_found())
    }


*/

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                document_formatting_provider: Some(OneOf::Left(true)),
                document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
                    first_trigger_character: ';'.to_string(), // not working
                    more_trigger_character: Some(vec!["\n".to_owned()]),
                }),
                position_encoding: Some(PositionEncodingKind::UTF16),
                inlay_hint_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: Some(vec![CodeActionKind::QUICKFIX]),
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: Some(true),
                        },
                        resolve_provider: Some(true),
                    },
                )),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::INCREMENTAL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_owned()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
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
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        SemanticTokensRegistrationOptions {
                            text_document_registration_options: {
                                TextDocumentRegistrationOptions {
                                    document_selector: Some(vec![DocumentFilter {
                                        language: Some("norg".to_owned()),
                                        scheme: Some("file".to_owned()),
                                        pattern: None,
                                    }]),
                                }
                            },
                            semantic_tokens_options: SemanticTokensOptions {
                                work_done_progress_options: WorkDoneProgressOptions::default(),
                                legend: SemanticTokensLegend {
                                    token_types: neorg_syntax::highlight::LEGEND_TYPE.into(),
                                    token_modifiers: vec![],
                                },
                                range: None,
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                            },
                            static_registration_options: StaticRegistrationOptions::default(),
                        },
                    ),
                ),
                ..ServerCapabilities::default()
            },
        })
    }
    async fn initialized(&self, _: InitializedParams) {
        eprintln!("initialized!");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let (key, text) = (
            params.text_document.uri.to_string(),
            params.text_document.text,
        );

        // insert new doc into document map
        self.document_map.insert(key.clone(), text.into());

        // Update the CST map with the new CST
        if let Some(ctx) = self.document_map.get(&key) {
            let cst = neorg_syntax::cst!(&ctx.to_string());
            self.cst_map.insert(key, cst);
        }

        //  if let Ok(diagnostics) = self.provide_diagnostics(params.text_document.uri.clone()) {
        //      // Publish the diagnostics to the client
        //      self.client
        //          .publish_diagnostics(params.text_document.uri.clone(), diagnostics, None)
        //          .await;
        //  }
        //  self.client
        //      .log_message(
        //          MessageType::INFO,
        //          format!("Opened file: {}", text_document.uri),
        //      )
        //      .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.on_change(params).await
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let mut res = Vec::new();
        let key = params.text_document.uri.to_string();

        if let Some(text) = self.document_map.get(&key)
            && let Some(new_text) = neorg_syntax::cst!(&text.to_string()).format()
        {
            let lines_count = text.lines().count();
            let start = Position {
                line: 0,
                character: 0,
            };
            let end = if let Some(last_line) = text.lines().last() {
                Position {
                    line: lines_count as u32 - 1,
                    character: last_line.chars().count() as u32,
                }
            } else {
                start
            };
            let range = Range { start, end };
            res.push(TextEdit { range, new_text });
        }
        dbg!(&res);
        Ok(Some(res))
    }

    async fn on_type_formatting(
        &self,
        p: DocumentOnTypeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        eprintln!("pressed trigger characters!");
        let p = DocumentFormattingParams {
            text_document: p.text_document_position.text_document,
            options: p.options,
            work_done_progress_params: WorkDoneProgressParams {
                work_done_token: None,
            },
        };
        self.formatting(p).await
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(_text) = params.text {
            //  self.on_change(item).await;
            _ = self.client.semantic_tokens_refresh().await;
        }
        eprintln!("file saved!");
    }
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let key = params.text_document.uri.to_string();
        self.document_map.remove(&key);
        self.cst_map.remove(&key);
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        use crate::handle::HandleDefinition;
        match self.provide_def_ctx(params).await {
            Ok(def) => Ok(def),
            Err(_) => Ok(None),
        }
    }

    async fn references(&self, _params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        Ok(None)
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let key = params.text_document.uri.to_string();
        let tokens = self
            .cst_map
            .get(&key)
            .map_or(SemanticTokens::default(), |node| {
                tower_lsp::lsp_types::SemanticTokens {
                    result_id: None,
                    data: node.collect_semantic_tokens(),
                }
            });

        Ok(Some(SemanticTokensResult::Tokens(tokens)))
    }

    async fn semantic_tokens_range(
        &self,
        _params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        Ok(None)
    }

    async fn inlay_hint(
        &self,
        _params: tower_lsp::lsp_types::InlayHintParams,
    ) -> Result<Option<Vec<InlayHint>>> {
        let mut hints = Vec::new();
        let test = InlayHint {
            position: Position {
                line: 1,
                character: 2,
            },
            label: InlayHintLabel::String("01".to_owned()),
            kind: Some(InlayHintKind::TYPE),
            padding_left: Some(true),
            padding_right: Some(true),
            data: None,
            text_edits: None,
            tooltip: None,
        };
        hints.push(test);
        Ok(Some(hints))
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(None)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        Ok(None)
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        eprintln!("configuration changed!");
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        eprintln!("workspace folders changed!");
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        eprintln!("watched files have changed!");
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        eprintln!("command executed!");

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    /// Handle hover requests
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        use crate::handle::HandleHover;
        self.provide_hover_ctx(params).await
    }

    /// Handle code action requests
    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        use crate::handle::HandleCodeAction;
        self.provide_code_action(params).await
    }
}

#[allow(unused)]
enum CustomNotification {}
impl Notification for CustomNotification {
    type Params = InlayHintParams;
    const METHOD: &'static str = "custom/notification";
}

#[allow(ungated_async_fn_track_caller)]
impl Backend {
    #[track_caller]
    async fn on_change(&self, params: DidChangeTextDocumentParams) {
        // 01. get key
        // 02. get mutated Ropey for that key
        // 03. update Ropey from vec of changes in params
        let key = params.text_document.uri.to_string();

        if let Some(mut doc) = self.document_map.get_mut(&key) {
            for change in params.content_changes {
                // Get the document content
                let rope = doc.value_mut();

                // Get the range of the change
                if let Some(range) = change.range {
                    // Get the start and end positions of the range
                    // Convert the position to an offset
                    let start_idx = position_to_offset(range.start, rope).unwrap_or(0);
                    let end_idx =
                        position_to_offset(range.end, rope).unwrap_or(rope.len_utf16_cu());
                    // Replace the text in the range with the new text
                    rope.remove(start_idx..end_idx);
                    rope.insert(start_idx, &change.text);
                } else {
                    // If range is None, replace the whole text
                    rope.remove(0..rope.len_chars());
                    rope.insert(0, &change.text);
                }
            }
        }

        // let rope = ropey::Rope::from_str(params.text);
        // self.document_map
        //     .insert(params.uri.to_string(), rope.clone());
        // let uri = params.uri.as_str();

        // == diagnostics ==
        let _diagnostics = self.get_diagnostics(&key);
        if let Some(source) = self.document_map.get(&key) {
            let source = source.to_string();
            let mut p = neorg_syntax::Parser::new(&source);
            let parsed = neorg_syntax::document(&mut p);
            self.cst_map.insert(key.to_owned(), parsed);
        }
    }
}

pub fn offset_to_position(offset: usize, rope: &Rope) -> Option<Position> {
    let line = rope.try_char_to_line(offset).ok()?;
    let first_char_of_line = rope.try_line_to_char(line).ok()?;
    let column = offset - first_char_of_line;
    Some(Position::new(line as u32, column as u32))
}

pub fn position_to_offset(position: Position, rope: &Rope) -> Option<usize> {
    let line_char_offset = rope.try_line_to_char(position.line as usize).ok()?;
    let slice = rope.slice(0..line_char_offset + position.character as usize);
    Some(slice.len_utf16_cu())
}
