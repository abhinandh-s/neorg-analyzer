/// A string scanner.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Scanner<'a> {
    /// The string to scan.
    string: &'a str,
    /// The index at which we currently are. To guarantee safety, it must always
    /// hold that:
    /// - 0 <= cursor <= string.len()
    /// - cursor is on a character boundary
    cursor: usize,
}

impl<'a> Scanner<'a> {
    /// Creates a new string Scanner, starting with a cursor position of `0`.
    #[inline]
    pub fn new(string: &'a str) -> Self {
        Self { string, cursor: 0 }
    }

    /// The full source string.
    #[inline]
    pub fn string(&self) -> &'a str {
        self.string
    }

    /// The current cursor position.
    #[inline]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Whether the scanner has fully consumed the string.
    #[inline]
    pub fn done(&self) -> bool {
        self.cursor == self.string.len()
    }

    /// The subslice before the cursor.
    ///```
    /// #[inline]
    /// pub fn before(&self) -> &'a str {
    ///     // Safety: The cursor is always in-bounds and on a codepoint boundary.
    ///     debug_assert!(self.string.is_char_boundary(self.cursor));
    ///     unsafe { self.string.get_unchecked(..self.cursor) }
    /// }
    /// ```
    #[inline]
    pub fn before(&self) -> &'a str {
        &self.string[..self.cursor]
    }

    /// // The subslice after the cursor.
    ///```
    /// #[inline]
    /// pub fn after(&self) -> &'a str {
    ///     // Safety: The cursor is always in-bounds and on a codepoint boundary.
    ///     debug_assert!(self.string.is_char_boundary(self.cursor));
    ///     unsafe { self.string.get_unchecked(self.cursor..) }
    /// }
    /// ```
    #[inline]
    pub fn after(&self) -> &'a str {
        &self.string[self.cursor..]
    }

    /// // The subslices before and after the cursor.
    /// ```
    /// #[inline]
    /// pub fn parts(&self) -> (&'a str, &'a str) {
    ///     (self.before(), self.after())
    /// }
    /// ```
    #[inline]
    pub fn parts(&self) -> (&'a str, &'a str) {
        (self.before(), self.after())
    }

    /// ```
    /// #[inline]
    /// pub fn peek(&self) -> Option<char> {
    ///     self.after().chars().next()
    /// }
    /// ```
    ///
    /// The character right behind the cursor.
    #[inline]
    pub fn peek(&self) -> Option<char> {
        self.after().chars().next()
    }
}

pub trait ScanOpts {
    fn eat(&mut self) -> Option<char>;
    fn uneat(&mut self) -> Option<char>;
}

impl ScanOpts for Scanner<'_> {
    /// Consume and return the character right behind the cursor.
    #[inline]
    fn eat(&mut self) -> Option<char> {
        let peeked = self.peek();
        if let Some(char) = peeked {
            // Safety: When `c` is right behind the cursor, there must be an
            // in-bounds codepoint boundary at `self.cursor + c.len_utf8()`.
            self.cursor += char.len_utf8();
        }
        peeked
    }

    fn uneat(&mut self) -> Option<char> {
        todo!()
    }
}
