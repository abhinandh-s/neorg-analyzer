use neorg_syntax::parser::Lexer;

fn main() -> anyhow::Result<(), anyhow::Error> {
    let input = include_str!("../../../examples/test.norg");
    
    for i in Lexer::lex(input) {
        println!("{:?}", i);
    }

    //while let token = l.next_token() {
    //    if token == Token::Eof {
    //        break;
    //    }
    //    println!("{:?}", token);
    //}
    Ok(())
}
