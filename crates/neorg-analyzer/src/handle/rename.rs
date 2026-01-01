
impl crate::backend::Backend {
    
}

// pub(crate) trait HandleRename {
//     async fn provide_def_ctx(
//         &self,
//         params: GotoDefinitionParams,
//     ) -> Result<Option<tower_lsp::lsp_types::GotoDefinitionResponse>, Error>;
// }

// impl HandleDefinition for Backend {
//     async fn provide_def_ctx(
//         &self,
//         params: GotoDefinitionParams,
//     ) -> Result<Option<tower_lsp::lsp_types::GotoDefinitionResponse>, Error> {
//         let mut path = "/home/abhi/.local/share/neorg/dict/".to_owned();
//         if !Path::new(&path).exists() {
//             let _ = tokio::fs::create_dir_all(&path).await;
//         }
//         let uri = params
//             .text_document_position_params
//             .text_document
//             .uri
//             .to_string();
//
//         if let Some(s) = self.cst_map.get(uri.as_str()) {
//             let node = s.to_owned();
//             let words = neorg_syntax::get_kinds(neorg_syntax::SyntaxKind::Word, node);
//             for word in words {
//                 if contains_pos(word.range(), params.text_document_position_params.position) {
//                     path += word.text();
//                     path += ".md";
//                     let _cached_result = tokio::fs::read_to_string(&path).await;
//                     let search_result = get_meaning(word.text()).await;
//                     match search_result {
//                         Ok(meaning) => {
//                             let _ = tokio::fs::write(&path, meaning).await;
//                         }
//                         _ => return Ok(None),
//                     }
//                 }
//             }
//         }
//         match Url::from_file_path(&path) {
//             Ok(uri) => Ok(Some(GotoDefinitionResponse::Scalar(Location {
//                 uri,
//                 range: neorg_syntax::cst!(&path).range(),
//             }))),
//             Err(_) => Ok(None),
//         }
//     }
// }
//

