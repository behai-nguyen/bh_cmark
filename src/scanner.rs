/* 26/03/2026. */

// Major refactoring to be carried out.
//
// See D:\Codes\Documentation\markdown_scanner_refactoring_whitespace_token.md
//
// All spaces are to be Whitespace `Token`s.
//
// Save backup before refactoring.

//! The Markdown scanner implementation. 
//! 
//! # Example
//! 
//! ```rust
//! let markdown: &str = r"**bold \\Úc Đại Lợi\\**";
//! 
//! let mut scanner = Scanner::new(markdown);
//! let res = scanner.scan_tokens();
//! 
//! assert!(res.is_ok(), "{} should be valid", "scanning");
//! 
//! let tokens = res.unwrap();
//! for token in &tokens {
//!     println!("{:?},", token);
//! }
//! ```

use std::{iter::Peekable, str::Chars};

use super::token_type::TokenType;
use super::token::Token;

static BLOCK_ELEMENT_BREAK_CHARACTERS: &[char] = 
    &['#', '*', '_', '!', '[', ']', '(', ')', '\r', '\n', '\\'];
    //&['#', '*', '_', '!', '[', ']', '(', ')', '\n', '\\'];

/// # Note
/// 
/// ```text
/// Before emitting a token:
///     start fields already point at token start.
/// 
/// After consuming chars:
///    end fields point at token end.
/// ```
/// 
/// Therefore:
/// 
/// * source_byte_start/logical_byte_start: beginning of current token.
/// 
/// * source_byte_end/logical_byte_end: one-past-the-end of consumed input.
/// 
/// # Assumption and Future Exensibility
/// 
/// It is implicitly assumed that:
/// 
///     **Adjacent text tokens are semantically contiguous**.
/// 
/// This is currently true. However, if later introduced:
/// 
///     * trivia,
///     * hidden whitespace,
///     * soft breaks,
///     * entity decoding,
///     * normalisation passes,
/// 
/// will require “mergeable text” vs “boundary-preserving text”.
/// 
pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    break_characters: &'static [char],
    source_byte_start: usize,
    source_byte_end: usize,
    logical_byte_start: usize,
    logical_byte_end: usize,
    line: usize,
}

pub type ScanResult<T> = Result<T, String>;

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: source,
            chars: source.chars().peekable(),
            break_characters: BLOCK_ELEMENT_BREAK_CHARACTERS,
            source_byte_start: 0,
            source_byte_end: 0,
            logical_byte_start: 0,
            logical_byte_end: 0,
            line: 1,
        }
    }

    fn set_byte_starts(&mut self, source_count: usize, logical_count: usize) {
        self.source_byte_start = source_count;
        self.logical_byte_start = logical_count;
    }

    fn inc_byte_ends(&mut self, c: &char, include_logical: bool) {
        self.source_byte_end += c.len_utf8();
        if include_logical {
            self.logical_byte_end += c.len_utf8();
        }
    }

    fn inc_line(&mut self) {
        self.line += 1;
    }

    /// Push a [`Token`] onto the parameter `token_vector`.
    /// 
    /// # Arguments
    /// 
    /// * `token_vector`: the running [`Token`] vector.
    /// 
    /// * `lexeme`: the [`Token`]'s `lexeme`.
    /// 
    /// * `token_type`: the [`Token`]'s `token_type`.
    /// 
    /// # Pre-Conditions
    /// 
    /// `self.source_byte_start`, `self.source_byte_end`, `self.logical_byte_start`, 
    /// and `self.logical_byte_end` have been calculated correctly up to the `lexeme`
    /// sub-string.
    /// 
    fn add_token_with_lexeme(&mut self, 
        token_vector: &mut Vec<Token>, 
        lexeme: &str, 
        token_type: TokenType,
    ) {
        if token_type == TokenType::Text {
            if let Some(last) = token_vector.last_mut() {
                if last.token_type() == TokenType::Text {
                    last.push_to_lexeme(lexeme);
                    last.inc_byte_ends(lexeme.len(), lexeme.len());
                    return;
                }
            }
        }

        token_vector.push(Token::new(token_type, lexeme.to_string(), self.source_byte_start, 
            self.source_byte_end, self.logical_byte_start, self.logical_byte_end, self.line));        
    }

    /// Extract the [`Token`]'s `lexeme` which has been already identified 
    /// within the `self.source_byte_start` and `self.source_byte_end` byte 
    /// range, and push it to the parameter `token_vector`.
    /// 
    /// # Arguments
    /// 
    /// * `token_vector`: the running [`Token`] vector.
    /// 
    /// * `token_type`: [`TokenType`].
    /// 
    fn add_token(&mut self, 
        token_vector: &mut Vec<Token>, 
        token_type: TokenType,
    ) {
        let lex = &self.source[self.source_byte_start..self.source_byte_end];
        self.add_token_with_lexeme(token_vector, lex, token_type);
    }

    fn is_escapable(&self, c: char) -> bool {
        // The list below matches entries in BLOCK_ELEMENT_BREAK_CHARACTERS.
        matches!(c, '#' | '*' | '_' | '!' | '[' | ']' | '(' | ')' | '\r' | '\n' | '\\')
    }    

    /// Identify and collect literal text until hitting one of the [`Token`] 
    /// character.
    /// 
    /// # Arguments
    /// 
    ///  * `token_vector`: the running [`Token`] vector.
    /// 
    /// # Returns
    ///
    /// * Always [`Ok()`].
    /// 
    fn text(&mut self, 
        token_vector: &mut Vec<Token>
    ) -> ScanResult<()> {
        while let Some(c) = self.chars.peek() {
            // Not in escaped state, and the current character is a marker for a 
            // block element, e.g. `!`; or an inline element, e.g. `*`. The text 
            // token has ended. Return the caller, it is the caller's responsiblity 
            // to deal with this character.
            // Note that `self.chars.peek()` does not advance, the caller still 
            // sees this current marker character.
            if self.break_characters.contains(&c) {
                break;
            }

            // Spaces and Tabs are `Token`s of type `Whitespace`. They could sit in 
            // the `self.break_characters` list, but this separated block makes it 
            // much more visible, making the intention clearer.
            if [' ', '\t'].contains(c)  {
                break;
            }

            let ch = c.clone();
            self.inc_byte_ends(&ch, true);

            self.chars.next();
        }

        self.add_token(token_vector, TokenType::Text);

        Ok(())
    }

    fn consume_newline(&mut self, 
        token_vector: &mut Vec<Token>
    ) -> ScanResult<()> {
        // The `\r` character: included in the final rendered text.
        self.inc_byte_ends(&'\r', true);

        // Move pass `\r`.
        self.chars.next();

        if self.chars.peek() == Some(&'\n') {
            self.inc_byte_ends(&'\n', true);
            // Consume '\n'.
            self.chars.next();
        }

        self.add_token(token_vector, TokenType::Newline);

        self.inc_line();

        Ok(())
    }

    /// Process the escape marker `\\` and next character in the source 
    /// inptut text.
    /// 
    /// # Arguments
    /// 
    ///  * `token_vector`: the running [`Token`] vector.
    /// 
    /// # Returns
    ///
    /// * Always [`Ok()`].
    fn escape(&mut self, 
        token_vector: &mut Vec<Token>
    ) -> ScanResult<()> {
        // Move pass `\\`.
        self.chars.next();

        // Check out the next character after the escape marker `\`. If there 
        // is not a next character, ignore the escape marker `\` entirely.
        if let Some(c) = self.chars.next() {
            let ch = c.clone();

            if self.is_escapable(ch) {
                // Escape marker is ignored in final rendering.
                self.inc_byte_ends(&'\\', false);
                self.set_byte_starts(self.source_byte_end, self.logical_byte_end);

                self.inc_byte_ends(&ch, true);

                self.add_token(token_vector, TokenType::EscapedChar);

            } else {
                // Literal `\` character: included in the final rendered text.
                self.inc_byte_ends(&'\\', true);
                self.set_byte_starts(self.source_byte_end, self.logical_byte_end);

                // The actual character follows the literal `\` which has been 
                // iterated over in the second call `self.chars.next()` above.
                self.inc_byte_ends(&ch, true);

                self.add_token_with_lexeme(token_vector, 
                    &format!("\\{ch}"), TokenType::Text);
            }
        }

        Ok(())
    }

    /// Advance `self.byte_index`, add a [`Token`], and move to next character 
    /// in the source input Markdown text.
    /// 
    /// # Arguments
    /// 
    ///  * `token_vector`: the running [`Token`] vector.
    /// 
    ///  * `token_char`: the character to increase `self.byte_index` by.
    /// 
    ///  * `token_type`: the [`TokenType`] of the new [`Token`].
    /// 
    fn manage_adding_token(&mut self, 
        token_vector: &mut Vec<Token>, 
        token_char: &char, 
        token_type: TokenType) {
        self.inc_byte_ends(token_char, true);
        self.add_token(token_vector, token_type);

        self.chars.next();
    }

    /// Process the current character in the source input Markdown text.
    /// 
    /// # Arguments
    /// 
    ///  * `token_vector`: the running [`Token`] vector.
    /// 
    /// # Returns
    ///
    /// * Always [`Ok()`].
    /// 
    fn scan_token(&mut self, token_vector: &mut Vec<Token>) -> ScanResult<()> {
        let c: char = *self.chars.peek().unwrap_or(&'\0');

        match c {
            '#' => self.manage_adding_token(token_vector, &c, TokenType::Hash),
            '*' => self.manage_adding_token(token_vector, &c, TokenType::Star), 
            '_' => self.manage_adding_token(token_vector, &c, TokenType::Underscore),
            '-' => self.manage_adding_token(token_vector, &c, TokenType::Dash),
            '!' => self.manage_adding_token(token_vector, &c, TokenType::Bang),
            '[' => self.manage_adding_token(token_vector, &c, TokenType::LBracket),
            ']' => self.manage_adding_token(token_vector, &c, TokenType::RBracket),
            '(' => self.manage_adding_token(token_vector, &c, TokenType::LParen),
            ')' => self.manage_adding_token(token_vector, &c, TokenType::RParen),
            '\r' => self.consume_newline(token_vector)?,
            '\n' => {
                self.inc_line();
                self.manage_adding_token(token_vector, &c, TokenType::Newline);
            }
            '\\' => self.escape(token_vector)?,
            ' ' | '\t' => self.manage_adding_token(token_vector, &c, TokenType::Whitespace),
            _ => self.text(token_vector)?,
        }

        Ok(())
    }

    /// Scan the source input Markdown text to produce a syntactically 
    /// meaningful vector of [`Token`].
    /// 
    /// # Arguments
    /// 
    ///  * `token_vector`: the running [`Token`] vector.
    /// 
    /// # Returns
    ///
    /// * [`Vec<Token>`] — on success.
    /// 
    /// * Error messages separated by `\n` — on failure.
    /// 
    pub fn scan_tokens(&mut self) -> ScanResult<Vec<Token>> {
        if self.chars.peek().is_none() {
            return Err("Source text is empty.".into());
        }
        
        let mut token_vector = Vec::<Token>::new();
        let mut err_msgs: Vec<String> = vec![];

        while let Some(_) = self.chars.peek() {
            // We are at the beginning of the next lexeme: set the new start byte.
            self.set_byte_starts(self.source_byte_end, self.logical_byte_end);

            match self.scan_token(&mut token_vector) {
                Ok(_) => {},
                Err(err) => err_msgs.push(format!("{}", err)),
            }
        }

        let eof_token = || -> Token {
            Token::new(TokenType::Eof, "".to_string(), 
                self.source_byte_end, self.source_byte_end, 
                self.logical_byte_end, self.logical_byte_end, 
                self.line)
        };

        if err_msgs.len() == 0 {
            token_vector.push(eof_token());
            Ok(token_vector)
        } else {
            Err(err_msgs.join("\n").into())
        }
    }
}