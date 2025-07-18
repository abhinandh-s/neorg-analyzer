use serde::Deserialize;
use tower_lsp::jsonrpc::Error;
use tower_lsp::lsp_types::{
    Hover, HoverContents, HoverParams, MarkupContent, MarkupKind, Position, Range,
};

use crate::backend::Backend;

mod diagnostics;

pub(crate) trait HandleHover {
    async fn provide_hover_ctx(&self, params: HoverParams) -> Result<Option<Hover>, Error>;
}

impl HandleHover for Backend {
    async fn provide_hover_ctx(&self, params: HoverParams) -> Result<Option<Hover>, Error> {
        let mut hover_ctx = String::new();
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let Position { line, character } = params.text_document_position_params.position;

        if let Some(s) = self.cst_map.get(uri.as_str()) {
            let node = s.to_owned();
            let words = neorg_syntax::get_kinds(neorg_syntax::SyntaxKind::Word, node);
            for word in words {
                let Range { start, end } = word.range();
                let (start_line, start_char) = (start.line, start.character);
                let (last_line, last_char) = (end.line, end.character);

                if (line >= start_line)
                    && (line <= last_line)
                    && (character >= start_char)
                    && (character <= last_char)
                {
                    hover_ctx.push_str("hover_ctx for: ");
                    if let Ok(meaning) = get_meaning(word.text()).await {
                        hover_ctx.push_str(&meaning);
                    }
                    // let ctx = format!("hover_ctx for `{}`", meaning);
                    // hover_ctx.push_str(&ctx);
                    // hover_ctx.push_str("this is content");
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

#[derive(Debug, Deserialize)]
struct DictionaryEntry {
    meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,

    definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize)]
struct Definition {
    definition: String,
    example: Option<String>,
}

async fn get_meaning(word: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");
    let response = reqwest::get(&url).await?;
    let entries = response.json::<Vec<DictionaryEntry>>().await?;

    let mut result = String::new();

    if let Some(entry) = entries.first() {
        if let Some(meaning) = entry.meanings.first() {
            if let Some(def) = meaning.definitions.first() {
                result.push_str(&format!("**Definition**: {}\n", def.definition));
                if let Some(example) = &def.example {
                    result.push_str(&format!("**Example**: _{}_", example));
                }
            }
        }
    }

    Ok(result)
}

