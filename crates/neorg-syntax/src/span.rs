// Span information for tracking source positions
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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
    pub fn detached(&self) -> Self {
        Self {
            start: 1,
            end: 1,
            line: 1,
            column: 1,
        }
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

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used, unused)]

    use super::lsp::LspRange;
    use crate::parse::Parser;

    #[test]
    fn to_lsp_range_test() {
        use super::*;

        let input = r#"# headings
*italics*
**bold**

## bold
        "#;
        let mut parser = Parser::new(input);
        let (cst, _ast) = parser.parse().unwrap();
        let _span = cst.span();
        let headings = cst.find_by_kind(crate::kind::SyntaxKind::Heading);

        let one = headings[0];
        let lsp_range = one.span().to_lsp_range();

        assert_eq!(lsp_range.start.line, 1);
        assert_eq!(lsp_range.start.character, 1);
        assert_eq!(lsp_range.end.line, 1);
        assert_eq!(lsp_range.end.character, 10);

        let two = headings[1];
        let lsp_range = two.span().to_lsp_range();

        assert_eq!(lsp_range.start.line, 5);
        assert_eq!(lsp_range.start.character, 32);
        assert_eq!(lsp_range.end.line, 5);
        assert_eq!(lsp_range.end.character, 38);

        let bold = cst.find_by_kind(crate::kind::SyntaxKind::Bold);
        for i in bold {
            let lsp_range = i.span().to_lsp_range();
            assert_eq!(lsp_range.start.line, 3);
            assert_eq!(lsp_range.start.character, 22);
            assert_eq!(lsp_range.end.line, 3);
            assert_eq!(lsp_range.end.character, 29);
        }

        let italics = cst.find_by_kind(crate::kind::SyntaxKind::Italic);
        for i in italics {
            let lsp_range = i.span().to_lsp_range();
            assert_eq!(lsp_range.start.line, 2);
            assert_eq!(lsp_range.start.character, 12);
            assert_eq!(lsp_range.end.line, 2);
            assert_eq!(lsp_range.end.character, 20);
        }
    }
}
