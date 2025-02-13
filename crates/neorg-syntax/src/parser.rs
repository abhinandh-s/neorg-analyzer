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
    pub fn lex(input: &'a str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut l = Self::new(input);
        while l.next_token() != Token::Eof {
            tokens.push(l.next_token().clone());
        }
        tokens
    }

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

            // Markup
            Some('/') => self.read_markup_token(Token::Italic),
            Some('_') => self.read_markup_token(Token::Underline),
            Some('!') => self.read_markup_token(Token::Spoiler),
            Some('`') => self.read_markup_token(Token::Verbatim),
            Some('%') => self.read_markup_token(Token::Comment),

            // Links
            Some('{') => {
                self.advance();
                Token::LinkStart
            }
            Some('}') => {
                self.advance();
                Token::LinkEnd
            }
            Some('[') => {
                self.advance();
                Token::LinkDescStart
            }
            Some(']') => {
                self.advance();
                Token::LinkDescEnd
            }

            // Default to reading text
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
    fn read_task_marker(&mut self) -> Token {
        self.advance(); // eat |
        if self.peek_char() == Some('(') {
            self.advance();
            Token::TaskMarkerStart
        } else {
            Token::Pipe
        }
    }

    #[inline]
    fn read_code_block(&mut self) -> Token {
        self.advance(); // eat @
        let text = self.read_until_whitespace();

        if text == "code" {
            self.skip_whitespace();
            let language = if !self.is_at_line_end() {
                Some(self.read_until_whitespace())
            } else {
                None
            };
            Token::CodeBlockStart { language }
        } else {
            // handle unknown @ directive as text
            Token::Text(format!("@{}", text))
        }
    }

    #[inline]
    fn read_ordered_list(&mut self) -> Token {
        let mut level = 0;
        while self.peek_char() == Some('~') {
            level += 1;
            self.advance();
        }
        Token::OrderedList { level }
    }

    #[inline]
    fn read_dash_token(&mut self) -> Token {
        let mut count = 0;
        while self.peek_char() == Some('-') {
            count += 1;
            self.advance();
        }

        // Check if it's a reverse heading
        if count >= 3 {
            Token::ReverseHeading { levels: 1 }
        } else {
            Token::UnorderedList { level: count }
        }
    }

    #[inline]
    fn read_markup_token(&mut self, token_type: Token) -> Token {
        self.advance();
        if self.peek_char() == Some('|') {
            self.advance();
            // Handle pipe-delimited markup
        }
        token_type
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
