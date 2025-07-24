#![allow(dead_code)]

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

#[derive(Debug, Deserialize)]
pub(crate) struct DictionaryEntry {
    pub(crate) word: String,
    pub(crate) phonetic: Option<String>,
    pub(crate) origin: Option<String>,
    pub(crate) meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub(crate) part_of_speech: String,
    pub(crate) definitions: Vec<Definition>,
}
impl From<&Meaning> for MarkDown {
    fn from(value: &Meaning) -> Self {
        let mut md = String::new();
        let heading = format!("## {}\n", value.part_of_speech);
        md.push_str(&heading);
        for definition in &value.definitions {
            let MarkDown(content) = definition.into();
            md.push_str(&content);
            md.push('\n');
        }
        Self(md)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Definition {
    pub(crate) definition: String,
    pub(crate) example: Option<String>,
}

impl From<&Definition> for MarkDown {
    fn from(value: &Definition) -> Self {
        let mut md = String::new();
        md.push_str(value.definition.as_ref());
        md.push_str("\n \n");
        if let Some(example) = &value.example {
            md.push_str("> Example: \n");
            md.push_str("> ");
            md.push_str(example.as_ref());
            md.push('\n');
        }
        Self(md)
    }
}

struct MarkDown(String);

impl From<&DictionaryEntry> for MarkDown {
    fn from(value: &DictionaryEntry) -> Self {
        let mut md = String::new();
        md.push_str("# ");
        md.push_str(value.word.as_str());
        if let Some(phonetic) = &value.phonetic {
            md.push('\t');
            md.push_str(phonetic);
        }
        md.push('\n');
        md.push('\n');
        if let Some(origin) = &value.origin {
            md.push_str("## Origin\n");
            md.push('\t');
            md.push_str(origin);
            md.push('\n');
        }
        for i in &value.meanings {
            let MarkDown(content) = i.into();
            md.push_str(&content);
        }
        Self(md)
    }
}

async fn get_meaning(word: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");
    let response = reqwest::get(&url).await?;
    let entries = response.json::<Vec<DictionaryEntry>>().await?;

    let mut result = String::new();

    if let Some(entry) = entries.first() {
        let MarkDown(content) = entry.into();
        result.push_str(&content);
    }

    Ok(result)
}
