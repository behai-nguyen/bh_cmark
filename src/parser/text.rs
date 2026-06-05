/* 26/05/2026. */

//! Collects a line of text, which can include emphases, and passes 
//! the [`crate::token::Token`] list to [`crate::parser::delimiter::DelimiterParser`]
//! to parse the inline text.

use std::collections::HashSet;

use crate::token_type::TokenType;
use crate::token::Token;
use crate::ast::AstBlock;
use crate::parser::base::{
    ParseResult,
    BaseParser,
};

/// Parses inline text via [`crate::parser::delimiter::DelimiterParser`].
/// 
/// # Arguments
/// 
/// * `base` — a mutable [`crate::parser::base::BaseParser`] which already 
///   contains the target [`crate::token::Token`] list.
/// 
/// # Return
/// 
/// [`crate::ast::AstBlock::Paragraph`] — on success.
/// 
/// [`std::error::Error`] — if some error occurs.
pub fn text(base: &mut BaseParser) -> ParseResult<AstBlock> {
    let is_new_line = |token: Option<&Token>| -> bool {
        match token {
            Some(t) => t.token_type() == TokenType::Newline,
            None => false,
        }
    };

    let start = base.current();
    let mut whitespaces_index: HashSet<usize> = HashSet::new();

    while let Some(token) = base.peek() {
        let mut token_type = token.token_type();
        
        // Consider `\n  ###   bar    ###`:
        //    `base.current()` points to the first `\n`, this is a start of line.
        //    Attempt to remove any leading space? TO_DO: thematic and code block.
        let mut whitespace_count: usize = 0;
        if matches!(token_type, TokenType::Whitespace) && 
            is_new_line(base.previous()) {
            whitespace_count += 1;
            whitespaces_index.insert(base.current());
            // Move pass the first `Whitespace` just encountered and counted.
            base.advance();
            while !base.is_at_end() {
                token_type = match base.peek() {
                    None => break,                
                    Some(token) => token.token_type(),
                };

                if matches!(token_type, TokenType::Whitespace) {
                    whitespace_count += 1;
                    whitespaces_index.insert(base.current());
                    base.advance();
                } else {
                    break;
                }
            }
        }

        let start_of_line = if whitespace_count < 4 
            { base.is_start_of_line() } else { false };

        if matches!(token_type, TokenType::Eof)
            || base.is_blank_line()
            || (start_of_line 
                && matches!(token_type, TokenType::Hash | TokenType::Bang))
        {
            break;
        }

        base.advance();
    }

    if start == base.current() {
        return Err("empty paragraph".into());
    }
    
    let block = AstBlock::paragraph(
        base.parse_inline_text(start, 
        base.current(), &whitespaces_index)?);

    // consume trailing newlines
    while matches!(
        base.peek().map(|t| t.token_type()),
        Some(TokenType::Newline)
    ) {
        base.advance();
    }

    Ok(block)
}
