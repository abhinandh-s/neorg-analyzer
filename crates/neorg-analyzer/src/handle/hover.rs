use std::path::Path;

use reqwest::Url;
use tower_lsp::jsonrpc::Error;
use tower_lsp::lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams, Location,
    MarkupContent, MarkupKind, Position, Range,
};

use crate::backend::Backend;
use crate::types::{DictionaryEntry, MarkDown};

pub(crate) fn contains_pos(r: Range, p: Position) -> bool {
    let Range { start, end } = r;
    let Position { line, character } = p;
    (line >= start.line)
        && (line <= end.line)
        && (character >= start.character)
        && (character <= end.character)
}

pub(crate) trait HandleHover {
    async fn provide_hover_ctx(&self, params: HoverParams) -> Result<Option<Hover>, Error>;
}

pub(crate) trait HandleDefinition {
    async fn provide_def_ctx(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<tower_lsp::lsp_types::GotoDefinitionResponse>, Error>;
}

impl HandleDefinition for Backend {
    async fn provide_def_ctx(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<tower_lsp::lsp_types::GotoDefinitionResponse>, Error> {
        let mut path = "/home/abhi/.local/share/neorg/dict/".to_owned();
        if !Path::new(&path).exists() {
            let _ = tokio::fs::create_dir_all(&path).await;
        }
        let key = params
            .text_document_position_params
            .text_document
            .uri
            .as_str();

        if let Some(sytaxnode) = self.cst_map.get(key) {
            let node = sytaxnode.to_owned();
            let words = neorg_syntax::get_kinds(neorg_syntax::SyntaxKind::Word, node);
            for word in words {
                if contains_pos(word.range(), params.text_document_position_params.position) {
                    path += word.text();
                    path += ".md";
                    let _cached_result = tokio::fs::read_to_string(&path).await;
                    let search_result = get_meaning(word.text()).await;
                    match search_result {
                        Ok(meaning) => {
                            let _ = tokio::fs::write(&path, meaning).await;
                        }
                        _ => return Ok(None),
                    }
                }
            }
        }
        match Url::from_file_path(&path) {
            Ok(uri) => Ok(Some(GotoDefinitionResponse::Scalar(Location {
                uri,
                range: neorg_syntax::cst!(&path).range(),
            }))),
            Err(_) => Ok(None),
        }
    }
}

impl HandleHover for Backend {
    async fn provide_hover_ctx(&self, params: HoverParams) -> Result<Option<Hover>, Error> {
        let mut hover_ctx = String::new();
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();

        if let Some(s) = self.cst_map.get(uri.as_str()) {
            let node = s.to_owned();
            let words = neorg_syntax::get_kinds(neorg_syntax::SyntaxKind::Word, node);
            for word in words {
                if contains_pos(word.range(), params.text_document_position_params.position)
                    && let Ok(meaning) = get_meaning(word.text()).await
                {
                    hover_ctx.push_str(&meaning);
                }
            }

            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: hover_ctx,
                }),
                range: None,
            }));
        }
        Ok(None)
    }
}

async fn get_meaning(word: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");
    let response = reqwest::get(&url).await?;
    let entries = response.json::<Vec<DictionaryEntry>>().await?;

    let mut result = String::new();

    if let Some(entry) = entries.first() {
        let content = std::convert::Into::<MarkDown>::into(entry);
        result.push_str(&content);
    }
    Ok(result)
}
