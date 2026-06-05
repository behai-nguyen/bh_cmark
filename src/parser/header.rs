/* 26/05/2026. */ 

//! Parses Markdown headers. 
//! 
//! Handles all "ATX headings" examples under 
//! https://spec.commonmark.org/0.31.2/spec.json correctly.

use std::collections::HashSet;

use crate::token_type::TokenType;
use crate::ast::{
    AstBlock,
    MAX_HEADER_LEVEL,
};
use crate::parser::base::{
    ParseResult,
    BaseParser,
};

fn discard_trailing_space(base: &mut BaseParser, 
    text_start: usize, 
    index: &mut usize
) {
    // Example 71: `## foo ##\n  ###   bar    ###\n`
    //    There can be more than one trailing space, get rid of all.
    while *index > text_start {
        match base.tokens().get(*index - 1) {
            Some(token) => {
                if matches!(token.token_type(), TokenType::Whitespace) {
                    *index -= 1;
                } else {
                    break;
                }
            },
            // This should not happen.
            _ => break,
        }
    }
}

/// Header text `Token` range has been worked out. Within this range, determine
/// what to remove and what to keep.
/// 
/// # Arguments
/// 
/// * `base` — the mutable [`BaseParser`] which holds the [`crate::token::Token`]
///   list.
/// 
/// * `text_start` — the index to the first header text [`crate::token::Token`].
/// 
/// * `text_end` — the index to the last header text [`crate::token::Token`].
fn clean_up_header_text(base: &mut BaseParser, 
    text_start: usize, 
    text_end: &mut usize
) {
    #[derive(Debug, Eq, PartialEq)]
    enum TokenState {
        None,
        Hash,
    }

    let mut token_state: TokenState = TokenState::None;
    let mut last_hash_index: usize = 0;

    while *text_end > text_start {
        let token = &base.tokens()[*text_end - 1];

        match token.token_type() {
            TokenType::Hash => {
                token_state = TokenState::Hash;
                if last_hash_index == 0 {
                    last_hash_index = *text_end;
                }
            }
            TokenType::Whitespace => {
                if token_state == TokenState::None {
                    // Discarding trailing spaces.
                } else if token_state == TokenState::Hash {
                    // Discarding closing header `#`s.
                    // `# foo## #` → `foo##`.
                    // `# foo## # ##` → `foo## #`.
                    // `# foo## # ## #####` → `foo## # ##`.
                    *text_end -= 1;
                    // Example 71: `## foo ##\n  ###   bar    ###\n`
                    //    There can be more than one trailing space, 
                    //    get rid of all.
                    discard_trailing_space(base, text_start, text_end);
                    break;
                }
            }
            _ => {                    
                if token_state == TokenState::None {
                    // Example 74: "### foo ### b\n":
                    //     Last `Token` `b` is a `Text` token.
                    //
                    // Partial Example 76: "# foo \\#\n": 
                    //     Last `Token` `#` is an `EscapedChar` token.
                } else if token_state == TokenState::Hash {
                    // Partial Example 76: "### foo \\###\n": 
                    //     The `Token` `\#` is an `EscapedChar` token, 
                    //     this syntax makes the whole `\###` part of 
                    //     the header text -- take all three `#`s.
                    *text_end = last_hash_index;
                }
                break;
            },
        }

        *text_end -= 1;
    }
}

/// Parses header via [`crate::parser::delimiter::DelimiterParser`].
/// 
/// # Note 
/// 
///   * See https://spec.commonmark.org/0.31.2/#atx-heading
/// 
/// # Arguments
/// 
/// * `base` — a mutable [`crate::parser::base::BaseParser`] which already 
///   contains the target [`crate::token::Token`] list.
/// 
/// # Return
/// 
/// [`crate::ast::AstBlock::Header`] — on success.
/// 
/// [`std::error::Error`] — if some error occurs.
pub fn header(base: &mut BaseParser) -> ParseResult<AstBlock> {
    // TokenType::Hash: should not cause an error.
    let hash_token = 
        base.consume(TokenType::Hash, "")?;

    let line_number = hash_token.line();

    // The `#` token which has just been consumed above.
    let mut level: u8 = 1;
    // Scan forward to count header level.
    while base.peek().map(|t| t.token_type()) == Some(TokenType::Hash) {
        level += 1; 
        base.advance();
    }

    // Check that there is a space or a newline after the last `#`.
    if let Some(token) = base.peek() {
        if !matches!(token.token_type(), TokenType::Whitespace | 
            TokenType::Newline | TokenType::Eof) {
            return Err(format!("Line {}: expected space after '#'", line_number));
        }
    }
    
    if level > MAX_HEADER_LEVEL as u8 {
        return Err(format!("Line {}: invalid header level {}", line_number, level));
    }

    // Getting rid of all leading `TokenType::Whitespace`s in 
    // the final header text.
    while matches!(base.peek().map(|t| t.token_type()), Some(TokenType::Whitespace)) {
        base.advance(); // Consume space.
    }

    // text_start inclusive.
    // text_end exclusive.
    let text_start = base.current();

    // Walk to the last `Token` of the header.
    while let Some(token) = base.peek() {
        if matches!(token.token_type(), TokenType::Newline | TokenType::Eof) {
            break;
        }

        base.advance();
    }

    // Now walk back to work out what to include as part of the header text 
    // and what to discard.

    // text_start inclusive.
    // text_end exclusive. Currently is the last `Token` index.
    let mut text_end = base.current();

    clean_up_header_text(base, text_start, &mut text_end);
    
    // Process the header text.
    return Ok(AstBlock::header(level, 
        base.parse_inline_text(text_start, 
            text_end, &HashSet::<usize>::new())?));
}
