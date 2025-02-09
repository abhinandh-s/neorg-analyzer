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

pub trait Scan {
    fn eat(&mut self) -> Option<char>;
    fn uneat(&mut self) -> Option<char>;
    fn eat_if<T>(&mut self, mut pat: impl Pattern<T>) -> bool;
    fn eat_while<T>(&mut self, mut pat: impl Pattern<T>) -> &'a str;
    fn eat_until<T>(&mut self, mut pat: impl Pattern<T>) -> &'a str;
    fn eat_whitespace(&mut self) -> &'a str;
}

impl Scan for Scanner<'_> {
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

    // Consume and return the character right before the cursor, moving it
    /// back.
    #[inline]
    fn uneat(&mut self) -> Option<char> {
        let unpeeked = self.before().chars().next_back();
        if let Some(c) = unpeeked {
            // Safety: When `c` is right before the cursor, there must be an
            // in-bounds codepoint boundary at `self.cursor - c.len_utf8()`.
            self.cursor -= c.len_utf8();
        }
        unpeeked
    }

/// Consume the given pattern if that's what's right behind the cursor.
    ///
    /// Returns `true` if the pattern was consumed.
    #[inline]
    fn eat_if<T>(&mut self, mut pat: impl Pattern<T>) -> bool {
        if let Some(len) = pat.matches(self.after()) {
            // Safety: The contract of `matches` guarantees that there is an
            // in-bounds codepoint boundary at `len` bytes into `self.after()`.
            self.cursor += len;
            true
        } else {
            false
        }
    }

    /// Consume while the given pattern is what's right behind the cursor.
    ///
    /// Returns the consumed substring.
    #[inline]
    fn eat_while<T>(&mut self, mut pat: impl Pattern<T>) -> &'a str {
        let start = self.cursor;
        while let Some(len @ 1..) = pat.matches(self.after()) {
            // Safety: The contract of `matches` guarantees that there is an
            // in-bounds codepoint boundary at `len` bytes into `self.after()`.
            self.cursor += len;
        }
        self.from(start)
    }

    /// Consume until the given pattern is what's right behind the cursor.
    ///
    /// Returns the consumed substring.
    #[inline]
    fn eat_until<T>(&mut self, mut pat: impl Pattern<T>) -> &'a str {
        let start = self.cursor;
        while !self.done() && pat.matches(self.after()).is_none() {
            self.eat();
        }
        self.from(start)
    }

    /// Consume all whitespace until the next non-whitespace character.
    ///
    /// Returns the consumed whitespace.
    #[inline]
    fn eat_whitespace(&mut self) -> &'a str {
        self.eat_while(char::is_whitespace)
    }
}
