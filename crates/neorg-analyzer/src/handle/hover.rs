use tower_lsp::jsonrpc::Error;
use tower_lsp::lsp_types::{
    Hover, HoverContents, HoverParams, MarkupContent, MarkupKind, Position, Range,
};

use crate::backend::Backend;
use crate::types::{DictionaryEntry, MarkDown};

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

                if (line >= start.line)
                    && (line <= end.line)
                    && (character >= start.character)
                    && (character <= end.character)
                {
                    if let Ok(meaning) = get_meaning(word.text()).await {
                        hover_ctx.push_str(&meaning);
                    }
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
