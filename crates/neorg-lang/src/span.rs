use std::fmt::{self, Debug, Formatter};

// Span information for tracking source positions
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    // start position of the span
    pub start: usize,
    // end position of the span
    pub end: usize,
    // line number of the start of the span
    pub line: usize,
    // column number of the start of the span
    pub column: usize,
}

impl Span {
    /// Create a new span with the given start and end positions
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
    /// Get the range of the span
    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }

    /// Get line colomn information for rope
    /// -- TEST: line_column
    pub fn line_column(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    /// gives a fake Span which is detached from any source
    pub fn detached() -> Self {
        Self {
            start: 1,
            end: 1,
            line: 1,
            column: 1,
        }
    }
}

/// A value with a span locating it in the source code.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Spanned<T> {
    /// The spanned value.
    pub v: T,
    /// The value's location in source code.
    pub span: Span,
}

impl<T> Spanned<T> {
    /// Create a new instance from a value and its span.
    pub fn new(v: T, span: Span) -> Self {
        Self { v, span }
    }

    /// Convert from `&Spanned<T>` to `Spanned<&T>`
    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned { v: &self.v, span: self.span }
    }

    /// Map the value using a function.
    pub fn map<F, U>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned { v: f(self.v), span: self.span }
    }
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.v.fmt(f)
    }
}

#[cfg(feature = "tower-lsp")]
pub mod lsp {
    use ropey::Rope;
    use tower_lsp::lsp_types::{Position, Range};

    use super::Span;
    pub trait LspRange {
        fn to_lsp_range(&self) -> Range;
        fn offset_to_position(offset: usize, rope: &Rope) -> Option<Position> {
            let line = rope.try_char_to_line(offset).ok()?;
            let first_char_of_line = rope.try_line_to_char(line).ok()?;
            let column = offset - first_char_of_line;
            Some(Position::new(line as u32, column as u32))
        }

        fn position_to_offset(position: Position, rope: &Rope) -> Option<usize> {
            let line_char_offset = rope.try_line_to_char(position.line as usize).ok()?;
            let slice = rope.slice(0..line_char_offset + position.character as usize);
            Some(slice.len_bytes())
        }
    }

    impl LspRange for Span {
        /// .
        // --FIX: start and end are the same
        fn to_lsp_range(&self) -> Range {
            // -- TEST: me
            let (start_idx, end_idx) = (self.range().start as u32, self.range().end as u32);
            Range {
                start: Position {
                    line: self.line as u32,
                    character: start_idx + 1,
                },
                end: Position {
                    line: self.line as u32,
                    character: end_idx,
                },
            }
        }
    }
}
