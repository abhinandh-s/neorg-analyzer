#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SyntaxKind {
    Document,
    Heading,
    Paragraph,
    Text,
    Bold,
    Italic,
    LineBreak,
    Error,
}
