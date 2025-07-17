use dashmap::DashMap;
use neorg_analyzer::backend::Backend;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend {client, document_map: DashMap::new() });
    Server::new(stdin, stdout, socket).serve(service).await;
}
