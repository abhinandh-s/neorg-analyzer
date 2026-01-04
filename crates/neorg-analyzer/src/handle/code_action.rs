use std::collections::{HashMap, HashSet};

use tower_lsp::{
    jsonrpc::Error,
    lsp_types::{
        CodeActionKind, CodeActionOrCommand, CodeActionParams, CodeActionResponse, Position, Range,
        TextEdit,
    },
};

use crate::types::DictionaryEntry;
use crate::{backend::Backend, range};

pub(crate) trait HandleCodeAction {
    async fn provide_code_action(
        &self,
        params: CodeActionParams,
    ) -> Result<Option<CodeActionResponse>, Error>;
}

impl HandleCodeAction for Backend {
    async fn provide_code_action(
        &self,
        params: CodeActionParams,
    ) -> Result<Option<CodeActionResponse>, Error> {
        let mut result = Vec::new();
        let uri = params.text_document.uri.to_string(); // uri str for getting CST

        let test_range = &params.range; // is the range of char under the cursor.
        assert_eq!(test_range.start.line, test_range.end.line); // so the start's and end's line and char position must be equal
        assert_eq!(test_range.start.character, test_range.end.character);

        let Position { line, character } = params.range.start;

        #[allow(unused_assignments)] // false positive
        let mut word_range = range!(); // Range default

        let err = format!("{:#?}", &params.range);
        eprintln!("{err}");

        if let Some(s) = self.cst_map.get(uri.as_str()) {
            let node = s.to_owned();
            let words = neorg_syntax::get_kinds(neorg_syntax::SyntaxKind::Word, node);
            for word in words {
                let Range { start, end } = word.range();
                let err = format!("words: start: {:#?}, end: {:#?}", &start, &end);
                eprintln!("{err}");

                if (line == start.line)
                    && (line == end.line)
                    && (character >= start.character)
                    && (character <= end.character)
                {
                    word_range = word.range();
                    let word_text = word.text();
                    if let Ok(meaning) = get_synonyms(word.text()).await {
                        meaning.iter().for_each(|st| {
                            let mut store = HashMap::new();
                            store.insert(
                                params.text_document.uri.clone(),
                                vec![TextEdit {
                                    range: word_range,
                                    new_text: st.to_owned(),
                                }],
                            );
                            result.push(CodeActionOrCommand::CodeAction(
                                tower_lsp::lsp_types::CodeAction {
                                    title: format!("Rewrite `{word_text}` as `{st}`."),
                                    kind: Some(CodeActionKind::QUICKFIX),
                                    edit: Some(tower_lsp::lsp_types::WorkspaceEdit {
                                        changes: Some(store.clone()),
                                        ..Default::default()
                                    }),
                                    is_preferred: Some(true),
                                    ..Default::default()
                                },
                            ));
                        });
                    }
                }
            }

            return Ok(Some(CodeActionResponse::from(result)));
        }
        Ok(None)
    }
}

async fn get_synonyms(word: &str) -> Result<HashSet<String>, reqwest::Error> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");
    let response = reqwest::get(&url).await?;
    let entries = response.json::<Vec<DictionaryEntry>>().await?;

    let mut set = std::collections::HashSet::new();

    for entry in entries {
        for meaning in &entry.meanings {
            set.extend(meaning.synonyms.clone());
            for def in &meaning.definitions {
                set.extend(def.synonyms.clone());
            }
        }
    }

    Ok(set)
}
