// #![deny(missing_docs)]
#![allow(clippy::only_used_in_recursion, clippy::nonminimal_bool)]

use std::fmt::{self, Display, Formatter};

use ropey::Rope;

use crate::error::MarkdownError;
use crate::format::{FormattingOptions, MarkdownFormatter};
use crate::node::{ASTNode, CSTNode};
use crate::span::Span;
use crate::validate::MarkdownValidator;

/// A parser for parsing markdown text into a CST and AST
#[derive(Debug)]
pub struct Parser {
    text: Rope,
    pos: usize,
    line: usize,
    column: usize,
}

/// Extra methods for working with characters in the parser
trait CharOps {
    /// Check if the parser has reached the end of the input
    fn is_eof(&self) -> bool;
    /// Peek character in the input without consuming it
    fn peek_char(&self) -> char;
    /// Consume the next character in the input
    fn consume_char(&mut self) -> char;
    /// Peek the next character in the input without consuming it
    fn peek_next_char(&self) -> char;
}

/// Implement the extra character methods for the parser
impl CharOps for Parser {
    /// Check if the parser has reached the end of the input
    fn is_eof(&self) -> bool {
        self.pos >= self.text.len_chars()
    }

    /// Peek character in the input without consuming it
    fn peek_char(&self) -> char {
        if self.is_eof() {
            '\0'
        } else {
            self.text.char(self.pos)
        }
    }

    /// Peek the next character in the input without consuming it
    fn peek_next_char(&self) -> char {
        if self.pos + 1 >= self.text.len_chars() {
            '\0'
        } else {
            self.text.char(self.pos + 1)
        }
    }

    /// Consume the next character in the input
    fn consume_char(&mut self) -> char {
        let ch = self.peek_char();
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        ch
    }
}

impl Parser {
    /// Create a new parser with the given input text
    pub fn new(text: &str) -> Self {
        Self {
            text: Rope::from_str(text),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Parse the input text into a CST and AST
    pub fn parse(&mut self) -> Result<(CSTNode, ASTNode), MarkdownError> {
        let (cst, _) = self.parse_document()?;
        let ast = self.cst_to_ast(&cst)?;
        Ok((cst, ast))
    }

    /// Parse the input text into a CST
    fn parse_document(&mut self) -> Result<(CSTNode, Span), MarkdownError> {
        let start = self.pos;
        let mut nodes = Vec::new();

        while !self.is_eof() {
            if self.peek_char() == '*' {
                let (node, _) = self.parse_heading()?;
                nodes.push(node);
            } else if self.peek_char() == '\n' {
                self.consume_char();
                // -- FIX: is line count is correct now
                //
                //   self.line += 1;
                self.column = 1;
            } else {
                let (node, _) = self.parse_paragraph()?;
                nodes.push(node);
            }
        }

        let span = Span::new(start, self.pos, self.line, self.column);
        Ok((
            CSTNode::Document {
                children: nodes,
                span,
            },
            span,
        ))
    }

    /// Parse a heading node
    fn parse_heading(&mut self) -> Result<(CSTNode, Span), MarkdownError> {
        let start = self.pos;
        let mut level = 0;

        // Count heading level
        while self.peek_char() == '*' && level < 6 {
            self.consume_char();
            level += 1;
        }

        // Skip whitespace after heading markers
        while self.peek_char().is_whitespace() && self.peek_char() != '\n' {
            self.consume_char();
        }

        let markers_span = Span::new(start, self.pos, self.line, self.column);
        let (content, _content_span) = self.parse_inline()?;

        let span = Span::new(start, self.pos, self.line, self.column);
        Ok((
            CSTNode::Heading {
                level: level as u8,
                content: vec![content],
                markers: markers_span,
                span,
            },
            span,
        ))
    }

    fn parse_paragraph(&mut self) -> Result<(CSTNode, Span), MarkdownError> {
        let start = self.pos;
        let mut nodes = Vec::new();

        while !self.is_eof() && self.peek_char() != '\n' {
            let (node, _) = self.parse_inline()?;
            nodes.push(node);
        }

        let span = Span::new(start, self.pos, self.line, self.column);
        Ok((
            CSTNode::Paragraph {
                content: nodes,
                span,
            },
            span,
        ))
    }

    fn parse_inline(&mut self) -> Result<(CSTNode, Span), MarkdownError> {
        match self.peek_char() {
            '*' => {
                if self.peek_next_char() == '*' {
                    self.parse_bold()
                } else {
                    self.parse_italic()
                }
            }
            '_' => {
                if self.peek_next_char() == '_' {
                    self.parse_bold()
                } else {
                    self.parse_italic()
                }
            }
            '\n' => {
                let start = self.pos;
                self.consume_char();
                let span = Span::new(start, self.pos, self.line, self.column);
                Ok((CSTNode::LineBreak { span }, span))
            }
            _ => self.parse_text(),
        }
    }

    fn parse_bold(&mut self) -> Result<(CSTNode, Span), MarkdownError> {
        let start = self.pos;
        let marker = self.peek_char();
        self.consume_char();
        self.consume_char();

        let markers_span = Span::new(start, self.pos, self.line, self.column);
        let mut content = Vec::new();

        while !self.is_eof() && !(self.peek_char() == marker && self.peek_next_char() == marker) {
            let (node, _) = self.parse_inline()?;
            content.push(node);
        }

        // Consume closing markers
        self.consume_char();
        self.consume_char();

        let span = Span::new(start, self.pos, self.line, self.column);
        Ok((
            CSTNode::Bold {
                content,
                markers: markers_span,
                span,
            },
            span,
        ))
    }

    fn parse_italic(&mut self) -> Result<(CSTNode, Span), MarkdownError> {
        let start = self.pos;
        let marker = self.peek_char();
        self.consume_char();

        let markers_span = Span::new(start, self.pos, self.line, self.column);
        let mut content = Vec::new();

        while !self.is_eof() && self.peek_char() != marker {
            let (node, _) = self.parse_inline()?;
            content.push(node);
        }

        // Consume closing marker
        self.consume_char();

        let span = Span::new(start, self.pos, self.line, self.column);
        Ok((
            CSTNode::Italic {
                content,
                markers: markers_span,
                span,
            },
            span,
        ))
    }

    fn parse_text(&mut self) -> Result<(CSTNode, Span), MarkdownError> {
        let start = self.pos;
        let mut content = String::new();

        while !self.is_eof() {
            let ch = self.peek_char();
            if ch == '*' || ch == '_' || ch == '\n' {
                break;
            }
            content.push(self.consume_char());
        }

        let span = Span::new(start, self.pos, self.line, self.column);
        Ok((
            CSTNode::Text {
                content,
                span,
            },
            span,
        ))
    }

    fn cst_to_ast(&self, cst: &CSTNode) -> Result<ASTNode, MarkdownError> {
        match cst {
            CSTNode::Document { children, .. } => {
                let mut ast_children = Vec::new();
                for child in children {
                    ast_children.push(self.cst_to_ast(child)?);
                }
                Ok(ASTNode::Document(ast_children))
            }
            CSTNode::Heading {
                level,
                content,
                markers: _,
                span: _,
            } => {
                let mut ast_content = Vec::new();
                for node in content {
                    ast_content.push(self.cst_to_ast(node)?);
                }
                Ok(ASTNode::Heading {
                    level: *level,
                    content: ast_content,
                })
            }
            CSTNode::Paragraph { content, span: _ } => {
                let mut ast_content = Vec::new();
                for node in content {
                    ast_content.push(self.cst_to_ast(node)?);
                }
                Ok(ASTNode::Paragraph(ast_content))
            }
            CSTNode::Text { content, span: _ } => Ok(ASTNode::Text(content.clone())),
            CSTNode::Bold {
                content,
                markers: _,
                span: _,
            } => {
                let mut ast_content = Vec::new();
                for node in content {
                    ast_content.push(self.cst_to_ast(node)?);
                }
                Ok(ASTNode::Bold(ast_content))
            }
            CSTNode::Italic {
                content,
                markers: _,
                span: _,
            } => {
                let mut ast_content = Vec::new();
                for node in content {
                    ast_content.push(self.cst_to_ast(node)?);
                }
                Ok(ASTNode::Italic(ast_content))
            }
            CSTNode::LineBreak { span: _ } => Ok(ASTNode::LineBreak),
            CSTNode::Error { text, error: _ } => Ok(ASTNode::Error(text.clone())),
        }
    }
}

// Implement Display for AST nodes
impl Display for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_options(f, &FormattingOptions::default())
    }
}

impl ASTNode {
    fn fmt_with_options(&self, f: &mut Formatter<'_>, options: &FormattingOptions) -> fmt::Result {
        match self {
            ASTNode::Document(nodes) => {
                for (i, node) in nodes.iter().enumerate() {
                    if i > 0 {
                        writeln!(f)?;
                        if options.preserve_empty_lines {
                            writeln!(f)?;
                        }
                    }
                    node.fmt_with_options(f, options)?;
                }
            }
            ASTNode::Heading { level, content } => {
                write!(f, "{} ", "#".repeat(*level as usize))?;
                for (i, node) in content.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    node.fmt_with_options(f, options)?;
                }
                writeln!(f)?;
            }
            ASTNode::Paragraph(_vec) => {}
            ASTNode::Text(_) => {}
            ASTNode::Bold(_vec) => {}
            ASTNode::Italic(_vec) => {}
            ASTNode::LineBreak => {}
            ASTNode::Error(_) => {}
        }
        Ok(())
    }
}

// Extended Parser implementation
impl Parser {
    pub fn validate(&self, node: &CSTNode) -> Result<(), MarkdownError> {
        let mut validator = MarkdownValidator::new();
        validator.validate(node)
    }

    pub fn format(
        &self,
        node: &CSTNode,
        options: FormattingOptions,
    ) -> Result<String, MarkdownError> {
        let mut formatter = MarkdownFormatter::new(options);
        formatter.format(node)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use crate::validate::ValidationRule;

    use super::*;

    #[test]
    fn test_basic_parsing() -> Result<(), Error> {
        let input = "# Heading 1\nThis is a paragraph with **bold** and *italic* text.";
        let mut parser = Parser::new(input);
        let (cst, ast) = parser.parse()?;

        // You can add assertions here to verify the parsed structure
        assert!(matches!(cst, CSTNode::Document { .. }));
        assert!(matches!(ast, ASTNode::Document(..)));

        Ok(())
    }
    #[test]
    fn test_validation() -> Result<(), Error> {
        let input = "# Heading\n\n## Second Heading\n\nParagraph";
        let mut parser = Parser::new(input);
        let (cst, _) = parser.parse()?;

        let mut validator = MarkdownValidator::new();
        assert!(validator.validate(&cst).is_ok());

        Ok(())
    }

    #[test]
    fn test_formatting() -> Result<(), Error> {
        let input = "# Heading\nText";
        let mut parser = Parser::new(input);
        let (cst, _) = parser.parse()?;

        let options = FormattingOptions::default();
        let formatted = parser.format(&cst, options)?;
        assert!(formatted.contains("# Heading"));

        Ok(())
    }

    #[test]
    fn test_custom_validation_rule() -> Result<(), Error> {
        let input = "# Very Long Heading That Should Fail Validation Because It Exceeds The Maximum Line Length Limit";
        let mut parser = Parser::new(input);
        let (cst, _) = parser.parse()?;

        let mut validator = MarkdownValidator::new();
        validator.add_rule(ValidationRule::new(
            "max_heading_length",
            |node| {
                if let CSTNode::Heading { content, .. } = node {
                    content
                        .iter()
                        .map(|n| match n {
                            CSTNode::Text { content, .. } => content.len(),
                            _ => 0,
                        })
                        .sum::<usize>()
                        <= 50
                } else {
                    true
                }
            },
            "Heading length should not exceed 50 characters",
        ));

        assert!(validator.validate(&cst).is_err());

        Ok(())
    }
}
