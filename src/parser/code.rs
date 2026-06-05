/* 30/05/2026. */

use crate::token_type::TokenType;
use crate::ast::AstBlock;
use crate::parser::base::{
    ParseResult,
    BaseParser,
};

/// Parses indented code block via [`crate::parser::delimiter::DelimiterParser`].
/// 
/// # Arguments
/// 
/// * `base` — a mutable [`crate::parser::base::BaseParser`] which already 
///   contains the target [`crate::token::Token`] list.
/// 
/// # Return
/// 
/// [`crate::ast::AstBlock::Code`] — on success.
/// 
/// [`std::error::Error`] — if some error occurs.
pub fn code_block(base: &mut BaseParser) -> ParseResult<AstBlock> {
    let mut content = String::new();

    while let Some(token) = base.peek() {
        let token_type = token.token_type();
        
        match token_type {
            TokenType::Newline => {
                content.push_str(token.lexeme());
                break;
            }
            TokenType::Eof => break,
            _ => content.push_str(token.lexeme()),
        }

        base.advance();
    }

    // TO_DO: no language for this iteration.
    return Ok(AstBlock::code("", content))
}