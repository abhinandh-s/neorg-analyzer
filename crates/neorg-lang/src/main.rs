#![allow(dead_code)]

use std::fmt::Display;

use ecow::EcoString;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone)]
pub enum SyntaxKind {
    HeadingMarker,
    Text,
    Slash,
    Space,
    Asterisk,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: SyntaxKind,
    pub text: EcoString,
    pub span: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}: [text: {:?}, span: {}]",
            self.kind, self.text, self.span
        )
    }
}

impl Token {
    pub fn new(kind: SyntaxKind, text: EcoString, span: usize) -> Self {
        Self { kind, text, span }
    }
    pub fn to_span(&self, span: usize) -> Span {
        Span::new(span, span + self.text.len())
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut ve = Vec::new();
    let inp = input
        .split_word_bound_indices()
        .collect::<Vec<(usize, &str)>>();
    for (span, text) in inp {
        let tok = match text {
            " " => SyntaxKind::Space,
            "*" => SyntaxKind::HeadingMarker,
            "/" => SyntaxKind::Slash,
            "\n" => SyntaxKind::Newline,
            _ => SyntaxKind::Text,
        };
        let token = Token::new(tok, text.into(), span);
        ve.push(token);
    }
    ve
}

fn main() {
    let input = "* heading\n this is a ";
    let tokens = tokenize(input);
    for _tok in &tokens {
        //    println!("{}", tok);
    }
    let mut ast = Parser::new(tokens);
    let parsed = ast.parse();
    println!("{:#?}", parsed);
}

#[derive(Debug)]
pub enum NodeKind {
    Heading,
    Paragraph,
    Text,
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub children: Vec<Node>,
    pub token: Option<Token>,
}

impl Node {
    pub fn new(kind: NodeKind, token: Option<Token>) -> Self {
        Self {
            kind,
            children: Vec::new(),
            token,
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.current).cloned();
        if token.is_some() {
            self.current += 1;
        }
        token
    }

    fn parse_heading(&mut self) -> Node {
        let mut node = Node::new(NodeKind::Heading, None);
        while let Some(token) = self.advance() {
            match token.kind {
                SyntaxKind::Newline => break,
                _ => node.children.push(Node::new(NodeKind::Text, Some(token))),
            }
        }
        node
    }

    fn parse_paragraph(&mut self) -> Node {
        let mut node = Node::new(NodeKind::Paragraph, None);
        while let Some(token) = self.advance() {
            match token.kind {
                SyntaxKind::Newline => break,
                _ => node.children.push(Node::new(NodeKind::Text, Some(token))),
            }
        }
        node
    }

    fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while let Some(token) = self.peek() {
            match token.kind {
                SyntaxKind::Asterisk => nodes.push(self.parse_heading()),
                _ => nodes.push(self.parse_paragraph()),
            }
        }
        nodes
    }

    fn heading(&mut self) {}
}

enum Repr {
    Heading(Heading),
    Bold(Bold),
}

enum Heading {
    Markers(Vec<Token>),
    Space(Token),
    Text(Vec<Token>),
    Newline(Token),
}
enum Bold {
    MarkersStr(Token),
    Text(Vec<Token>),
    MarkersEnd(Token),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_span_001() {
        let input = (0, "Heading");

        let new = Token::new(SyntaxKind::Space, input.1.into(), input.0);

        assert_eq!(Span::new(0, 7), new.to_span(input.0))
    }
}
