#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SyntaxKind {
    /// the root
    Document,

    Paragraph,
    Heading,
    Text,

    /// Styling
    Bold,
    Italic,
    
    LineBreak,
    Error,
    /// End of File
    Eof,

    /// Keywords
    Code,
    End,

    /// Comments
    LineComment,
    BlockComment,
}

impl SyntaxKind {
    /// Whether this is an error.
    pub fn is_error(self) -> bool {
        self == Self::Error
    }

    /// Is this node is a keyword.
    pub fn is_keyword(self) -> bool {
        matches!(self, Self::Code | Self::End)
    }

    pub fn is_trivia(self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment)
    }

    pub fn name(self) -> &'static str {
        match self {
            SyntaxKind::Document => "nil",
            SyntaxKind::Heading => "nil",
            SyntaxKind::Paragraph => "nil",
            SyntaxKind::Text => "nil",
            SyntaxKind::Bold => "nil",
            SyntaxKind::Italic => "nil",
            SyntaxKind::LineBreak => "nil",
            SyntaxKind::Error => "nil",
            SyntaxKind::End => "nil",
            SyntaxKind::LineComment => "nil",
            SyntaxKind::BlockComment => "nil",
            SyntaxKind::Eof => "nil",
            SyntaxKind::Code => "nil",
        }
    }
}
