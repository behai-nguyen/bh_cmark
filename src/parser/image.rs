/* 26/05/2026. */ 

//! Parses simple Markdown image block `![caption](image path)`. For example:
//! 
//! `![***Mount Fuji* / *富士山, ふじさ, Fujisan* / *Núi Phú Sỹ***](./img/fujisan.png)`

use std::collections::HashSet;

use crate::token_type::TokenType;
use crate::ast::{
    AstBlock,
    InlineContent, 
};
use crate::parser::base::{
    ParseResult,
    BaseParser,
};

/// Parses image block via [`crate::parser::delimiter::DelimiterParser`].
/// 
/// Image blocks are only recognised at start-of-line. Otherwise they are treated 
/// as literal text.
/// 
/// # Limitations
/// 
///     1. Working internal documentation `markdown_image_block_parser.md`.
/// 
///     2. Unable to parse any of the "Images" examples in 
///        https://spec.commonmark.org/0.31.2/spec.json.
/// 
///        This parser only supports `![caption](image path)` syntax.
/// 
/// # Arguments
/// 
/// * `base` — a mutable [`crate::parser::base::BaseParser`] which already 
///   contains the target [`crate::token::Token`] list.
/// 
/// # Return
/// 
/// [`crate::ast::AstBlock::Image`] — on success.
/// 
/// [`std::error::Error`] — if some error occurs.
pub fn image_block(base: &mut BaseParser) -> ParseResult<AstBlock> {
    // TokenType::Bang: should not cause an error.
    let bang_token = 
        base.consume(TokenType::Bang, "")?;

    let line_number = bang_token.line();
    let error_message = |message: &str| -> String {
        format!("Line {}: {}", line_number, message)
    };

    // Parse caption. Caption content can be multi-tokens.
    // Consume the first syntax-critical '['.
    base.consume(TokenType::LBracket, &error_message("expected '['"))?;

    // Gather caption value. Handles internal "nested" '[' and ']'.
    // Capture the start index of the alt text.
    let alt_start = base.current();
    let mut bracket_depth_counter: usize = 0;
    // Scan forward until we find the closing bracket ']'.
    while let Some(token) = base.peek() {
        match token.token_type() {
            // "Nested" '[' inside caption.
            TokenType::LBracket => bracket_depth_counter += 1,
            TokenType::RBracket => {
                if bracket_depth_counter > 0 {
                    bracket_depth_counter -= 1;
                } else {
                    break;
                }
            }
            _ => {},
        }
        
        base.advance();
    }

    if base.peek().is_none() {
        return Err(error_message("expected ']' after '['"));
    }
    
    // Capture the end index and consume the bracket.
    let alt_end = base.current();
    base.consume(TokenType::RBracket, 
        &error_message("expected ']' after '['"))?;

    // Parse path/URL. Path/URL content can be multi-tokens.
    // Consume the first syntax-critical '('.
    base.consume(TokenType::LParen, &error_message("expected '('"))?;

    // Gather path/URL value. Handles internal "nested" '(' and ')'.
    let mut paren_depth_counter: usize = 0;   
    let mut path = String::new();
    // Scan forward until we find the closing parenthesis.
    while let Some(token) = base.peek() {
        match token.token_type() {
            // "Nested" '(' inside path.
            TokenType::LParen => {
                paren_depth_counter += 1;
                path.push_str(token.lexeme());
            }
            TokenType::RParen => {
                if paren_depth_counter > 0 {
                    paren_depth_counter -= 1;
                    path.push_str(token.lexeme());
                } else {
                    break;
                }
            }
            // Might still miss ')', therefore it is safer to break 
            // also on `Newline` and `Eof`.
            TokenType::Newline | TokenType::Eof => break,
            _ => path.push_str(token.lexeme()),
        }

        base.advance();
    }

    if path.len() == 0 {
        return Err(error_message("missing image path"));
    }        
            
    base.consume(TokenType::RParen, 
        &error_message("expected ')' after '('"))?;

    // CommonMark allows all-space captions.
    // Assuming there is no caption or caption is all whitespaces.
    let mut non_blank_caption: bool = false;
    for i in alt_start..alt_end {
        if !matches!(base.tokens()[i].token_type(), TokenType::Whitespace) {
            non_blank_caption = true;
            break;
        }
    };

    // Has caption. The caption can be multiline.
    if non_blank_caption {
        let alt = base.parse_inline_text(
            alt_start, alt_end, 
            &HashSet::<usize>::new())?;

        Ok(AstBlock::image(path, alt))
    } else {
        Ok(AstBlock::image(path, InlineContent::new_empty()))
    }
}
