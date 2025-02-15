use thiserror::Error;

/// error type for markdown parsing
#[derive(Debug, Error)]
pub enum MarkdownError {
    #[error("Parsing error at position {pos}: {message}")]
    ParseError { pos: usize, message: String },
    #[error("Invalid nesting of blocks")]
    InvalidNesting,
    #[error("Unexpected end of input")]
    UnexpectedEOF,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Formatting error: {0}")]
    FormattingError(String),
}
