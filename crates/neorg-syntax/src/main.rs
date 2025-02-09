use neorg_syntax::parse::Parser;

fn main() -> anyhow::Result<(), anyhow::Error> {
    let input = include_str!("../../../examples/sample.norg");
    let mut parser = Parser::new(input);
    let (cst, _ast) = parser.parse()?;

    println!("{:#?}", cst);

    Ok(())
}

// println!("vec of semantic: {:#?}", s);
// let h = cst.find_by_kind(neorg_syntax::kind::SyntaxKind::Heading);
// for i in h {
//     println!("vec of to_lsp_range method of Heading: {:#?}", i.span().to_lsp_range());
//     println!("vec of range method of Heading: {:#?}", i.span().range());
// }

//format_my_input(&cst);
//
//let s = cst.find_by_kind(neorg_syntax::kind::SyntaxKind::Heading);
//for i in s.clone() {
//    // println!("{:#?}", s);
//    let mut rope = ropey::Rope::from_str(input);
//    let _inp = rope.slice(i.span().range());
//    rope.remove(i.text_span().start - 1..i.span().end);
//    // rope.insert(i.span().range().start, "this is soooooo FUN!");
//    rope.insert(i.text_span().start - 1, "This is soooooo FUN!");
//    // println!("span: {:#?}", i.span());
//    // println!("text_span: {:#?}", i.text_span().start);
//    // println!("-{}-", inp);
//    println!("{}", rope);
//}
//
//let _re = validate_this();
//
//let _re2 = symbol_table(input);

//fn validate_this() -> anyhow::Result<(), anyhow::Error> {
//    let input = include_str!("../../../examples/sample.neorg");
//
//    let mut parser = Parser::new(input);
//    let (cst, _ast) = parser.parse()?;
//
//    let mut validator = MarkdownValidator::new();
//    validator.add_rule(ValidationRule::new(
//        "max_heading_length",
//        |node| {
//            if let CSTNode::Heading { content, .. } = node {
//                content
//                    .iter()
//                    .map(|n| match n {
//                        CSTNode::Text { content, .. } => content.len(),
//                        _ => 0,
//                    })
//                    .sum::<usize>()
//                    <= 50
//            } else {
//                true
//            }
//        },
//        "Heading length should not exceed 50 characters",
//    ));
//
//    let re = validator.validate(&cst);
//    match re {
//        Ok(_) => println!("Validation passed"),
//        Err(e) => println!("{}", e),
//    }
//    Ok(())
//}
//
//fn format_my_input(input: &CSTNode) {
//    let mut f = MarkdownFormatter::new(FormattingOptions {
//        indent_spaces: 2,
//        line_width: 50,
//        preserve_empty_lines: true,
//        heading_style: HeadingStyle::Atx,
//        emphasis_style: EmphasisStyle::Asterisk,
//    });
//    if let Ok(s) = f.format(input) {
//        println!("{}", s);
//    }
//}
