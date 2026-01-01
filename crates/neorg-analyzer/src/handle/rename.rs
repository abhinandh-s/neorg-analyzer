use std::collections::HashMap;

use tower_lsp::lsp_types::{RenameParams, TextEdit, WorkspaceEdit};

impl crate::backend::Backend {
    pub(crate) fn provide_rename(&self, params: RenameParams) -> Option<WorkspaceEdit> {
        let mut results = HashMap::new();

        let uri = params.text_document_position.text_document.uri.as_str();

        if let Some(value) = self.cst_map.get_mut(uri) {
            let node = value.to_owned();
            let words = neorg_syntax::get_kinds(neorg_syntax::SyntaxKind::Word, node);
            for word in words {
                if super::hover::contains_pos(word.range(), params.text_document_position.position)
                {
                    results.insert(
                        params.text_document_position.text_document.uri,
                        vec![TextEdit {
                            range: word.range(),
                            new_text: params.new_name,
                        }],
                    );
                    return Some(WorkspaceEdit {
                        changes: Some(results),
                        document_changes: None, // is a Workspace Edit
                        change_annotations: None,
                    });
                }
            }
        }
        None
    }
}
