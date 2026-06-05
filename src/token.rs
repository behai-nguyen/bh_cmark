/* 26/03/2026 */

//! The Markdown token. 
//! 
//! The Markdown `r"**bold \\Úc Đại Lợi\\**"` results in the following [`Token`]s:
//! 
//! ```rust
//! Token { token_type: Star, lexeme: "*", source_byte_start: 0, source_byte_end: 1, logical_byte_start: 0, logical_byte_end: 1, line: 1 },
//! Token { token_type: Star, lexeme: "*", source_byte_start: 1, source_byte_end: 2, logical_byte_start: 1, logical_byte_end: 2, line: 1 },
//! Token { token_type: Text, lexeme: "bold \\Úc Đại Lợi\\", source_byte_start: 2, source_byte_end: 25, logical_byte_start: 2, logical_byte_end: 25, line: 1 },
//! Token { token_type: Star, lexeme: "*", source_byte_start: 27, source_byte_end: 28, logical_byte_start: 25, logical_byte_end: 26, line: 1 },
//! Token { token_type: Star, lexeme: "*", source_byte_start: 28, source_byte_end: 29, logical_byte_start: 26, logical_byte_end: 27, line: 1 },
//! Token { token_type: Eof, lexeme: "", source_byte_start: 29, source_byte_end: 29, logical_byte_start: 27, logical_byte_end: 27, line: 1 },
//! ```

use super::token_type::TokenType;

/// Token for supported Markdown Block Elements.
/// - `Hash` → lexeme = `"#"`
/// - `Star` → lexeme = `"*"`
/// - `Text` → lexeme = `"hello world"`
/// - `Newline` → lexeme = `"\n"`
/// - `Eof` → lexeme = `""`
/// - etc.
///   * Original source positions: `source_byte_start`, `source_byte_end`.
///   * Logical normalised positions: `logical_byte_start`, `logical_byte_end`.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Token {
    /// The actual token type enum.
    token_type: TokenType,
    /// The actual lexeme representing `TokenType`.
    lexeme: String,
    /// Original source position: start of the `lexeme`.
    source_byte_start: usize,
    /// Original source position: end start of the `lexeme`.
    source_byte_end: usize,
    /// Logical normalised positions mean escape markers are not included 
    /// in the `lexeme` byte range positions.
    /// Logical normalised position: start of the `lexeme`.
    logical_byte_start: usize,
    /// Logical normalised position: end of the `lexeme`.
    logical_byte_end: usize,
    /// The input `md` file line number.
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, 
        lexeme: String, 
        source_byte_start: usize,
        source_byte_end: usize,
        logical_byte_start: usize,
        logical_byte_end: usize,
        line: usize
    ) -> Self {
        Token { token_type, lexeme, source_byte_start, source_byte_end, 
            logical_byte_start, logical_byte_end, line }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn source_byte_start(&self) -> usize {
        self.source_byte_start
    }

    pub fn source_byte_end(&self) -> usize {
        self.source_byte_end
    }

    pub fn logical_byte_start(&self) -> usize {
        self.logical_byte_start
    }

    pub fn logical_byte_end(&self) -> usize {
        self.logical_byte_end
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn push_to_lexeme(&mut self, s: &str) {
        self.lexeme.push_str(s);
    }

    pub fn inc_byte_ends(&mut self, source_count: usize, logical_count: usize) {
        self.source_byte_end += source_count;
        self.logical_byte_end += logical_count;
    }
}