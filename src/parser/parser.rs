/* 02/04/2026. */

//! The main parser.

use std::collections::HashSet;

use crate::parser::base::{
    ParseResult,
    ParseOutput,
    BaseParser,
};
use crate::token_type::TokenType;
use crate::token::Token;
use crate::ast::AstBlock;

use crate::parser::block_detector::BlockDetector;

use crate::parser::{
    text,
    header,
    image,
    code,
};

pub struct Parser<'a> {
    base: BaseParser<'a>,
}

/// The "Transaction" pattern.
impl<'a> Parser<'a> {
    /// Tries to run a parsing function. 
    /// If it returns Err, the parser state is rolled back to where it started.
    fn transaction<T, F>(&mut self, f: F) -> ParseResult<T>
    where
        F: FnOnce(&mut Self) -> ParseResult<T>,
    {
        self.base.save();
        let error_len = self.base.error_messages().len();

        let result = f(self);

        if result.is_err() {
            self.base.restore()?;
            self.base.truncate_error_messages(error_len);
        }
        result
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            base: BaseParser::new(tokens),
        }
    }

    fn consume_as_parsed_text(&mut self) -> ParseResult<AstBlock> {
        let start = self.base.current();
        let end = self.base.consume_until_block_boundary();

        Ok(AstBlock::paragraph(
            self.base.parse_inline_text(start, end, 
                &HashSet::<usize>::new())?
        ))
    }

    /// See https://spec.commonmark.org/0.31.2/#atx-heading
    fn header(&mut self) -> ParseResult<AstBlock> {
        header::header(&mut self.base)
    }

    fn image_block(&mut self) -> ParseResult<AstBlock> {
        image::image_block(&mut self.base)
    }

    /// Parses the input Markdown into a [`ParseOutput`].
    ///
    /// # Error handling
    ///
    /// This method never returns a parsing failure.
    ///
    /// Syntax errors are collected and returned inside
    /// [`ParseOutput`] while parsing continues where possible.
    ///
    /// Invalid Markdown is generally recovered as
    /// [`crate::ast::AstBlock::Paragraph`] so the document
    /// remains renderable.
    ///
    /// Use [`ParseOutput::has_error()`] to determine whether
    /// any recoverable parsing errors occurred.
    /// 
    /// # Return
    /// 
    /// [`crate::parser::base::ParseOutput`].
    pub fn parse(&mut self) -> ParseOutput {
        let mut blocks: Vec<AstBlock> = vec![];

        self.base.clear_error_message();

        while !self.base.is_at_end() {
            let token_type = match self.base.peek() {
                None => break,                
                Some(token) => token.token_type(),
            };

            let before = self.base.current();

            match token_type {
                TokenType::Eof => break,
                // A newline between block elements: ignore it.
                TokenType::Newline => {
                    self.base.advance();
                    continue;
                }
                _ => {}
            }

            let block = if self.base.is_start_of_line() {
                match token_type {
                    TokenType::Hash => 
                        self.transaction(|p| p.header())
                            .or_else(|err| {
                                self.base.add_error_message(err);
                                self.consume_as_parsed_text()
                        }),
                    TokenType::Bang => 
                        self.transaction(|p| p.image_block())
                            .or_else(|err| {
                                self.base.add_error_message(err);
                                self.consume_as_parsed_text()
                        }),
                    TokenType::Whitespace | TokenType::Star | 
                    TokenType::Underscore | TokenType::Dash => {
                        match self.base.detect_block() {
                            BlockDetector::ThematicBreak => Ok(AstBlock::thematic()),
                            BlockDetector::IndentedCode => code::code_block(&mut self.base),
                            BlockDetector::None => {
                                // Consider this Markdown `    hello`, if for some buggy
                                // reason `detect_block()` returns `BlockDetector::None`,
                                // then all leading spaces get thrown away.
                                if matches!(token_type, TokenType::Whitespace) {
                                    self.base.skip_line_whitespace();
                                    continue;
                                } else {
                                    text::text(&mut self.base)
                                }
                            }
                        }
                    }
                    _ => text::text(&mut self.base),
                }
            } else {
                // Mid-line → always text.
                text::text(&mut self.base)
            };

            match block {                
                Ok(b) => blocks.push(b),
                Err(err) => self.base.add_error_message(err.to_string()),
            }

            // Ensure progress.
            if self.base.current() == before {
                self.base.advance();
            }
        }
        
        ParseOutput::new(blocks, self.base.error_messages().clone())
    }
}