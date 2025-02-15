#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tokens<'a> {
    input: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    start: Position,
    end: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    line: u32,
    character: u32,
}

static DETACHED_RANGE: Range = Range {
    start: Position {
        line: 1,
        character: 1,
    },
    end: Position {
        line: 1,
        character: 1,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParseError {
    UnmatchedDelimiter {
        expected: char,
        found: Option<char>,
        span: Range,
    },
    InvalidLinkSyntax {
        span: Range,
    },
    InvalidListIndentation {
        span: Range,
    },
    UnexpectedToken {
        found: char,
        span: Range,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Root(RootNode),
    Paragraph(ParagraphNode),
    Error(ErrorNode),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootNode {
    children: Vec<Node>,
    span: Range,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParagraphNode {
    children: Vec<Node>,
    span: Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ErrorNode {
    pub error: ParseError,
    pub span: Range,
}
