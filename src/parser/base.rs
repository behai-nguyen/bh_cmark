/* 08/04/2026. */

/* 
Back up before refactoring not to rely on Eof.
*/

//! Composition (The "Inner Struct" Pattern).
//! 
//! Using boundary-based slice parsing rather than sentinel-based stream 
//! parsing. Although `tokens: &'a [Token]` in [`crate::parser::parser::Parser`]
//! does have a sentinel [`crate::token_type::TokenType::Eof`], when 
//! encountering inline text, `Parser` will pass only the inline text's 
//! [`crate::token::Token`] slice to [`crate::parser::delimiter::DelimiterParser`],
//! where the sentinel is not present.

use std::collections::HashSet;

use crate::parser::block_detector::{
    BlockDetector,
    MarkerRunAnalyser,
};

use crate::token_type::TokenType;
use crate::token::Token;
use crate::ast::{
    AstBlock,
    InlineContent, 
};

use crate::parser::delimiter::DelimiterParser;

/// LF character.
pub const END_OF_INPUT_CHAR: char = '¶';

pub type ParseResult<T> = Result<T, String>;

#[derive(Debug)]
pub struct ParseOutput {
    blocks: Vec<AstBlock>,
    errors: Vec<String>,
}

impl ParseOutput {
    pub fn new(blocks: Vec<AstBlock>, errors: Vec<String>) -> Self {
        ParseOutput { blocks, errors }
    }

    pub fn has_error(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn blocks(&self) -> &[AstBlock] {
        &self.blocks
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }    

    /// Consumes the ParseOutput and returns the inner `blocks` vector.
    pub fn into_blocks(self) -> Vec<AstBlock> {
        self.blocks
    }    

    pub fn errors(&self) -> &[String] {
        &self.errors
    }
}

pub struct BaseParser<'a> {
    tokens: &'a [Token],
    current: usize,
    saved_positions: Vec<usize>,
    error_messages: Vec<String>,
}

impl<'a> BaseParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        BaseParser {
            tokens,
            current: 0,
            saved_positions: Vec::new(),
            error_messages: Vec::new(),
        }
    }

    pub fn has_error(&self) -> bool {
        self.error_messages.len() > 0
    }

    pub fn save(&mut self) {
        self.saved_positions.push(self.current);
    }

    pub fn restore(&mut self) -> ParseResult<()> {
        self.current = self
            .saved_positions.pop()
            .take()
            .ok_or("BaseParser internal restore error")?;

        Ok(())
    }

    pub fn clear_last_save(&mut self) {
        self.saved_positions.pop();
    }

    pub fn clear_error_message(&mut self) {
        self.error_messages.clear();
    }

    pub fn add_error_message(&mut self, err_msg: String) {
        self.error_messages.push(err_msg);
    }

    pub fn truncate_error_messages(&mut self, to_len: usize) {
        self.error_messages.truncate(to_len);
    }

    pub fn tokens(&self) -> &[Token] {
        self.tokens
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn error_messages(&self) -> &Vec<String> {
        &self.error_messages
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn peek_n(&self, n: usize) -> Option<&Token> {
        if self.current + n < self.tokens.len() {
            self.tokens.get(self.current + n)
        } else {
            None
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            let token = &self.tokens[self.current];
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }

    pub fn consume(&mut self, 
        expected: TokenType, 
        error_message: &str,
    ) -> ParseResult<&Token> {
        match self.peek() {
            // If the token matches, we KNOW advance() will return Some, 
            // so we can safely unwrap it here to simplify the return type.
            Some(token) if token.token_type() == expected => {
                Ok(self.advance().unwrap()) 
            }
            _ => Err(error_message.into())
        }
    }

    pub fn is_punctuation(&self, c: char) -> bool {
        !c.is_alphanumeric() && !c.is_whitespace()
    }

    pub fn tokens_slice(&self, start: usize, end: usize) -> &[Token] {
        &self.tokens()[start..end]
    }

    pub fn is_blank_line(&self) -> bool {
        match (self.peek(), self.peek_n(1)) {
            (Some(t1), Some(t2)) =>
                t1.token_type() == TokenType::Newline &&
                t2.token_type() == TokenType::Newline,
            _ => false,
        }
    }

    pub fn consume_until_block_boundary(&mut self) -> usize {
        while let Some(token) = self.peek() {
            match token.token_type() {
                TokenType::Eof => break,

                TokenType::Newline => {
                    if self.is_blank_line() {
                        break;
                    }
                    // Keep single newline inside paragraph.
                    self.advance(); 
                }
                _ => {
                    self.advance();
                }
            }
        }

        self.current()
    }

    pub fn is_start_of_line(&self) -> bool {
        if self.current == 0 {
            return true;
        }

        let mut index = self.current;
        while index > 0 {
            match self.tokens[index - 1].token_type() {
                // `index == 0`, consider ` ### foo\n  ## foo\n   # foo\n`
                //     The parser is on the first `#`, the first ` ` (space) 
                //     reduces `index` to 0, it is a start of line.                
                TokenType::Whitespace => index -= 1,        
                // Consider: `...\n  ###   bar    ###` -- 
                //     The parser is on the first `#`, walking back through the 
                //     leading spaces and encontering a newline.
                TokenType::Newline => return true,
                _ => return false,
            }
        }

        true
    }

    /// Currently recognises only: whitespace (' '), asterisk ("*"), 
    /// underscore ("_"), and dash ("-"); and 4-leading spaces for 
    /// code block.
    /// Not yet supporting: "```", "~~~".
    /// 
    /// # Note
    /// 
    /// Indented code blocks are recognised immediately after observing 
    /// four leading spaces.
    /// 
    /// # Arguments
    /// 
    /// * `self` — has to be mutable, since this method is called by 
    ///   a mutable `self` as well.
    /// 
    /// # Return
    /// 
    /// [`BlockDetector`].
    pub fn detect_block(&mut self) -> BlockDetector {
        let mut markers = MarkerRunAnalyser::new();
        let mut prev_token_type: TokenType = TokenType::None;

        // There is a possibility of rolling back.
        self.save();

        while !self.is_at_end() {
            let token_type = match self.peek() {
                None => break,                
                Some(token) => token.token_type(),
            };

            match token_type {
                // TO_DO: when implement list etc. this will need to change.
                TokenType::Whitespace => {
                    let count = markers.register(token_type);
                    if count == 4 && markers.only_one_run() {
                        // Move pass the 4th space.
                        self.advance();
                        self.clear_last_save();
                        return BlockDetector::IndentedCode;
                    }
                }
                TokenType::Star | TokenType::Underscore | TokenType::Dash => {
                    if prev_token_type == TokenType::None {
                        prev_token_type = token_type.clone();
                    } else {
                        if prev_token_type != token_type {
                            let _ = self.restore();
                            return BlockDetector::None;
                        }
                    }

                    markers.register(token_type);
                }
                TokenType::Newline => {
                    self.clear_last_save();
                    break;
                }
                TokenType::Eof => break,
                _ => {
                    let _ = self.restore();
                    return BlockDetector::None;
                }
            }            

            self.advance();
        };

        // No early `return`: encountered either a newline or an eof.
        if markers.non_whitespace_count() >= 3 { BlockDetector::ThematicBreak } 
            else { 
                let _ = self.restore();
                BlockDetector::None 
            }
    }

    pub fn skip_line_whitespace(&mut self) {
        while matches!(
            self.peek().map(|t| t.token_type()),
            Some(TokenType::Whitespace)
        ) {
            self.advance();
        }
    }

    pub fn parse_inline_text(&self, 
        start_token_index: usize, 
        end_token_index: usize,
        ignored_tokens: &HashSet<usize>,
    ) -> ParseResult<InlineContent> {
        let mut inline_parser = 
            DelimiterParser::new(
                self.tokens_slice(start_token_index, end_token_index),
                ignored_tokens
            );

        Ok(inline_parser.parse_inline()?)
    }
}