use unicode_segmentation::UnicodeSegmentation;

use crate::kind::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    graphemes: Vec<&'a str>,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            // input into graphemes with is extended option true.
            graphemes: input.graphemes(true).collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.is_eof() {
            return Token::Eof;
        }

        match self.peek_char() {
            Some('\n') => {
                self.advance();
                Token::Newline
            }
            Some('*') => self.read_heading(),
            Some('-') => self.read_dash_token(),
            Some('~') => self.read_ordered_list(),
            Some('@') => self.read_code_block(),
            Some('|') => self.read_task_marker(),
            Some(_) => self.read_text(),
            None => Token::Eof,
        }
    }
}

impl Lexer<'_> {
    #[inline]
    fn read_text(&mut self) -> Token {
        let mut text = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_special_char() | c.is_whitespace() | c.is_newline() {
                break;
            }
            text.push(c);
            self.advance();
        }
        Token::Text(text)
    }

    #[inline]
    fn read_heading(&mut self) -> Token {
        let mut level = 0;
        while self.peek_char() == Some('*') {
            level += 1;
            self.advance();
        }
        Token::Heading { level }
    }

    #[inline]
    fn read_task_marker(&self) -> Token {
        todo!()
    }

    #[inline]
    fn read_code_block(&self) -> Token {
        todo!()
    }

    #[inline]
    fn read_ordered_list(&self) -> Token {
        todo!()
    }

    #[inline]
    fn read_dash_token(&self) -> Token {
        todo!()
    }
}

pub trait Lex {
    fn peek_char(&self) -> Option<char>;
    fn is_at_line_end(&self) -> bool;
    fn is_eof(&self) -> bool;
    fn advance(&mut self);
    fn read_until_whitespace(&mut self) -> String;
    fn skip_whitespace(&mut self);
}

impl Lex for Lexer<'_> {
    #[inline]
    fn peek_char(&self) -> Option<char> {
        self.graphemes
            .get(self.position)
            .and_then(|g| g.chars().next())
    }

    #[inline]
    fn advance(&mut self) {
        if let Some(grapheme) = self.graphemes.get(self.position) {
            if *grapheme == "\n" {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += grapheme.chars().count();
            }
            self.position += 1;
        }
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if !c.is_whitespace() || c == '\n' {
                break;
            }
            self.advance();
        }
    }

    #[inline]
    fn read_until_whitespace(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                break;
            }
            result.push(c);
            self.advance();
        }
        result
    }

    #[inline]
    fn is_at_line_end(&self) -> bool {
        self.peek_char().map_or(true, |f| f == '\n')
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.position == self.graphemes.len()
    }
}

pub trait NeorgChar {
    fn is_newline(&self) -> bool;
    fn is_special_char(&self) -> bool;
}

impl NeorgChar for char {
    #[inline]
    fn is_newline(&self) -> bool {
        *self == '\n'
    }
    #[inline]
    fn is_special_char(&self) -> bool {
        matches!(
            self,
            '*' | '-' | '~' | '@' | '|' | '/' | '_' | '!' | '`' | '%' | '{' | '}' | '[' | ']'
        )
    }
}
