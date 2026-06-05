/* 26/03/2026 */

use std::fmt;

/// Token type for supported Markdown Block Elements.
#[derive(Default, Debug, Clone, Hash, Eq, PartialEq)]
pub enum TokenType {
    #[default]
    /// Not defined yet.
    None,
    /// `#`, Header marker.
    Hash, 
    /// ' '. Spaces that follow `#`.
    Whitespace,
    /// Any character that follows the escape marker.
    /// Only ASCII punctuation characters are escapable.
    /// `\a` and `\中` are not escapes: they are all literal text.
    EscapedChar,
    /// `*`, Emphasis marker. I.e. `<em>..</em>` and `<strong>...</strong>`.
    Star,
    /// `_`, not yet fully implemented.
    Underscore,
    /// `-`, not yet fully implemented.
    Dash,
    /// `!`, Image block marker. I.e. `![(abc)](./img/test.png)`.
    Bang,
    /// `[`, In this version:
    ///     * Image block caption opening.
    ///     * Not implemented yet: `[GitHub](https://github.com)`
    ///     * Not implemented yet: `- [ ] Task item`, a task list or checkbox.
    LBracket,
    /// ']', In this version: 
    ///     * Image block caption closing.
    ///     * Not implemented yet: `[GitHub](https://github.com)`.
    ///     * Not implemented yet: `- [ ] Task item`, a task list or checkbox.
    RBracket,
    /// `(`, In this version: 
    ///     * Image block path/url opening.
    ///     * Not implemented yet: `[GitHub](https://github.com)`.
    LParen,
    /// ')', In this version: 
    ///     * Image block path/url closing.
    ///     * Not implement yet: `[GitHub](https://github.com)`.
    RParen,
    /// Any text.
    Text,
    /// `\n`.
    Newline,
    /// End-of-file marker.
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}