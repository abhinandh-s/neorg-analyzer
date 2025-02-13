#![allow(dead_code)]

#[derive(Debug)]
pub struct Tokens<'a> {
    input: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Range {
    start: Position,
    end: Position
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    line: u32,
    character: u32,
}

static STATIC: Range = Range {
    start: Position {
        line: 1,
        character: 1,
    },
    end: Position {
        line: 1,
        character: 1,
    },
};
