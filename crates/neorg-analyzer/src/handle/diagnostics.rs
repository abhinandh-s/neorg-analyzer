use neorg_syntax::get_diagnostics;
use tower_lsp::lsp_types::Diagnostic;

use crate::backend::Backend;

impl Backend {
    pub(crate) fn get_diagnostics(&self, uri: &str) -> Vec<Diagnostic> {
        self.cst_map
            .get(uri)
            .map_or(vec![], |node| get_diagnostics(node.to_owned()))
    }
}
