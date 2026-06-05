/* 12/04/2026 */

//! Test [`bh_cmark::parser::delimiter::DelimiterParser`]'s 
//! `match_delimiters()` method.
//! 
//! Test only inline Markdown texts, not block elements.
//! 
//! To run test for this module only:
//!
//!     * cargo test --test test_delimiter_match_delimiters
//!
//! To run a specific test method:
//!
//!     * cargo test test_delimiter_parser_match_delimiters -- --exact [--nocapture]
//!

use std::collections::HashSet;

use bh_cmark::scanner::Scanner;
use bh_cmark::parser::delimiter::{
    DelimiterParser,
    DelimiterRunVector,
    DelimiterMatchVector,
};

mod common;
use common::test_constant::*;

/// Mirroring [`bh_cmark::parser::delimiter::            TestRun`] struct with 
/// only fields that the [`DelimiterParser`]'s `match_delimiters()` method 
/// calculates values for, namely `self_index`.
/// 
/// ./tests/test_delimiter_pre_process.rs' `TestRun` tests other fields.
#[cfg(test)]
#[derive(Debug)]
struct TestRun {
    token_index: usize,
    count: u8,
    can_open: bool,
    can_close: bool,
    remaining: u8,
}

/// Mirroring [`bh_cmark::parser::delimiter::            TestMatch`].
#[cfg(test)]
#[derive(Debug)]
pub struct TestMatch {
    opener_index: usize,
    closer_index: usize,
    use_count: u8,   // 1 or 2
}

#[derive(Debug)]
struct TestInputAndResult<'a> {
    source: &'a str,
    expected_run_len: usize,
    expected_runs: &'static [TestRun],
    expected_match_len: usize,
    expected_matches: &'static [TestMatch],
    unique_tag: &'a str,
}

static PLAIN_TEXT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: HEADER_02_TEXT,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "PLAIN_TEXT_TESTS::HEADER_02_TEXT",
    },
    TestInputAndResult {
        source: TOKEN_LEXEME_AS_TEXT_01,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "PLAIN_TEXT_TESTS::TOKEN_LEXEME_AS_TEXT_01",
    },
];

static HASH_ESCAPE_TESTS: &'static [TestInputAndResult] = &[
    TestInputAndResult {
        source: HASH_ESCAPE_01,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HASH_ESCAPE_TESTS::HASH_ESCAPE_01",
    },
    TestInputAndResult {
        source: HASH_ESCAPE_02,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HASH_ESCAPE_TESTS::HASH_ESCAPE_02",
    },
];

static ASTERISK_TESTS: &'static [TestInputAndResult] = &[
    TestInputAndResult {
        source: ASTERISK_01,
        expected_run_len: 6,
        expected_runs: &[
            TestRun { token_index: 2, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 9, count: 2, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 21, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 25, count: 1, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 34, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 40, count: 3, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 4,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 2, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 4, closer_index: 5, use_count: 2 },
            TestMatch { opener_index: 4, closer_index: 5, use_count: 1 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_01",
    },
    TestInputAndResult {
        source: ASTERISK_02,
        expected_run_len: 7,
        expected_runs: &[
            TestRun { token_index: 2, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 9, count: 2, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 21, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 25, count: 1, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 34, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 40, count: 3, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 44, count: 1, can_open: true, can_close: true, remaining: 1 }, 
        ],
        expected_match_len: 4,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 2, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 4, closer_index: 5, use_count: 2 },
            TestMatch { opener_index: 4, closer_index: 5, use_count: 1 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_02",
    },
    TestInputAndResult {
        source: ASTERISK_03,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 7, count: 3, can_open: false, can_close: true, remaining: 1 }, 
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 }
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_03",
    },
    TestInputAndResult {
        source: ASTERISK_04,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 1 }, 
            TestRun { token_index: 8, count: 2, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 }            
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_04",
    },
    TestInputAndResult {
        source: ASTERISK_05,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 2 }, 
            TestRun { token_index: 8, count: 1, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 }
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_05",
    },
    TestInputAndResult {
        source: ASTERISK_06,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "ASTERISK_TESTS::ASTERISK_06",
    },    
    TestInputAndResult {
        source: ASTERISK_07,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 3, can_open: true, can_close: true, remaining: 0 }, 
            TestRun { token_index: 8, count: 2, can_open: true, can_close: true, remaining: 0 }, 
            TestRun { token_index: 11, count: 2, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 3,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
            TestMatch { opener_index: 2, closer_index: 3, use_count: 2 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_07",
    },
    TestInputAndResult {
        source: ASTERISK_08,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 3, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_08",
    },
    TestInputAndResult {
        source: ASTERISK_09,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "ASTERISK_TESTS::ASTERISK_09",
    },
    TestInputAndResult {
        source: ASTERISK_10,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 7, count: 2, can_open: true, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 }
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_10",
    },
    TestInputAndResult {
        source: ASTERISK_11,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "ASTERISK_TESTS::ASTERISK_11",
    },
    TestInputAndResult {
        source: ASTERISK_12,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 11, count: 2, can_open: true, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 }
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_12",
    },
    TestInputAndResult {
        source: ASTERISK_20,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 3, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 10, count: 2, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 13, count: 2, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 3,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
            TestMatch { opener_index: 2, closer_index: 3, use_count: 2 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_20",
    },
    TestInputAndResult {
        source: ASTERISK_21,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 3, can_open: true, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_21",
    },
    TestInputAndResult {
        source: ASTERISK_22,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 3, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_22",
    },
    TestInputAndResult {
        source: ASTERISK_23,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 3, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_23",
    },
    TestInputAndResult {
        source: ASTERISK_24,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 3, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_24",
    },
    TestInputAndResult {
        source: ASTERISK_25,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 3, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_25",
    },
    TestInputAndResult {
        source: ASTERISK_26,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 5, can_open: true, can_close: false, remaining: 1 }, 
            TestRun { token_index: 10, count: 4, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_26",
    },
];

static NESTED_ASTERISK_TESTS: &'static [TestInputAndResult] = &[
    TestInputAndResult {
        source: ASTERISK_13,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 0, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 10, count: 1, can_open: false, can_close: true, remaining: 0 },
            TestRun { token_index: 13, count: 2, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 3, use_count: 2 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
        ],
        unique_tag: "NESTED_ASTERISK_TESTS::ASTERISK_13",
    },
    TestInputAndResult {
        source: ASTERISK_14,
        expected_run_len: 6,
        expected_runs: &[
            TestRun { token_index: 0, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 6, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 8, count: 1, can_open: false, can_close: true, remaining: 0 },
            TestRun { token_index: 22, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 26, count: 1, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 28, count: 2, can_open: true, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 3,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 5, use_count: 2 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
            TestMatch { opener_index: 3, closer_index: 4, use_count: 1 },
        ],
        unique_tag: "NESTED_ASTERISK_TESTS::ASTERISK_14",
    },
    TestInputAndResult {
        source: EMPHASIS_BOLD_INSIDE_ITALIC_01,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 0, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 11, count: 2, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 16, count: 1, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 2 },
        ],
        unique_tag: "NESTED_ASTERISK_TESTS::EMPHASIS_BOLD_INSIDE_ITALIC_01",
    },
    TestInputAndResult {
        source: EMPHASIS_BOLD_INSIDE_ITALIC_02,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 4, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 8, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 15, count: 2, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 20, count: 1, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 2 },
        ],
        unique_tag: "NESTED_ASTERISK_TESTS::EMPHASIS_BOLD_INSIDE_ITALIC_02",
    },
];

// These are the bug fixed in https://github.com/behai-nguyen/polyglot_pdf/blob/main/pdf_06_text_styling/src/inline_parser.rs
// Addressed in this iteration. That is they produce the same output as VSC, and etc.
static BUG_ASTERISK_TESTS: &'static [TestInputAndResult] = &[
    TestInputAndResult {
        source: ASTERISK_15, 
        expected_run_len: 5,
        expected_runs: &[
            TestRun { token_index: 3, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 9, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 17, count: 1, can_open: false, can_close: true, remaining: 0 },
            TestRun { token_index: 20, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 24, count: 3, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 3,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 4, use_count: 2 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
            TestMatch { opener_index: 3, closer_index: 4, use_count: 1 },
        ],
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_15",
    },
    TestInputAndResult {
        source: ASTERISK_16,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 3, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_16",
    },
    TestInputAndResult {
        source: ASTERISK_17,
        expected_run_len: 5,
        expected_runs: &[
            TestRun { token_index: 0, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 6, count: 1, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 9, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 11, count: 3, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 3,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 4, use_count: 2 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
            TestMatch { opener_index: 3, closer_index: 4, use_count: 1 },
        ],
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_17",
    },
    TestInputAndResult {
        source: ASTERISK_18,
        expected_run_len: 3,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 1, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 7, count: 2, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 2, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_18",
    },
    TestInputAndResult {
        source: ASTERISK_19,
        expected_run_len: 3,
        expected_runs: &[
            TestRun { token_index: 1, count: 1, can_open: false, can_close: true, remaining: 1 }, 
            TestRun { token_index: 3, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 10, count: 3, can_open: false, can_close: true, remaining: 1 }, 
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 1, closer_index: 2, use_count: 2 }
        ],
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_19",
    },
];

static SPECIAL_ESCAPE_TESTS: &'static [TestInputAndResult] = &[
    TestInputAndResult {
        source: RECURRING_ESCAPE_01,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "SPECIAL_ESCAPE_TESTS::RECURRING_ESCAPE_01",
    },
    TestInputAndResult {
        source: ESCAPE_NEWLINE_01,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "SPECIAL_ESCAPE_TESTS::ESCAPE_NEWLINE_01",
    },
    TestInputAndResult {
        source: ESCAPE_INSIDE_EMPHASIS_01,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "SPECIAL_ESCAPE_TESTS::ESCAPE_INSIDE_EMPHASIS_01",
    },
    TestInputAndResult {
        source: EMPHASIS_ADJACENT_ESCAPE_01,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 4, count: 1, can_open: true, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 }
        ],
        unique_tag: "SPECIAL_ESCAPE_TESTS::EMPHASIS_ADJACENT_ESCAPE_01",
    },
];

static EMPHASIS_UNEVEN_TOKEN_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_01,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 1, count: 2, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 14, count: 2, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 }
        ],
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_01",
    },
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_02,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 2, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 13, count: 2, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 }
        ],
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_02",
    },
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_03,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 1, count: 2, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 14, count: 2, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 }
        ],
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_03",
    },
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_04,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 2, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 13, count: 2, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 }
        ],
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_04",
    },
];

// These are the captions of some image blocks defined above, these captions 
// are special, such as: multilines, recurring escape, emphasis, etc. They 
// are defined here to test the [`bh_cmark::parser::delimiter::DelimiterParser`] 
// various methods.
static HEADER_CAPTION_TEXT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: MULTILINE_LINE_CAPTION_01,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTILINE_LINE_CAPTION_01",
    },
    TestInputAndResult {
        source: MULTILINE_LINE_CAPTION_02,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTILINE_LINE_CAPTION_02",
    },
    TestInputAndResult {
        source: MULTILINE_LINE_CAPTION_03,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTILINE_LINE_CAPTION_03",
    },
    TestInputAndResult {
        source: MULTI_LINGUAL_CAPTION_01,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTI_LINGUAL_CAPTION_01",
    },
    TestInputAndResult {
        source: WIN_STYLE_PATH_TEXT,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::WIN_STYLE_PATH_TEXT",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_01,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_01",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_02,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_02",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_03,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[], 
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_03",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_04,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_04",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_05,
        expected_run_len: 0,
        expected_runs: &[],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_05",
    },
    TestInputAndResult {
        source: EMPHASIS_CAPTION_01,
        expected_run_len: 6,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 6, count: 1, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 10, count: 1, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 16, count: 1, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 20, count: 1, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 26, count: 3, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 4,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 5, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
            TestMatch { opener_index: 2, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 4, closer_index: 5, use_count: 1 },
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::EMPHASIS_CAPTION_01",        
    },
    TestInputAndResult {
        source: NESTED_EMPHASIS_CAPTION_01,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 5, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 7, count: 1, can_open: false, can_close: true, remaining: 0 },
            TestRun { token_index: 10, count: 3, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 3,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 3, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::NESTED_EMPHASIS_CAPTION_01",
    },
    TestInputAndResult {
        source: EMPHASIS_ESCAPE_CAPTION_01,
        expected_run_len: 6,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 6, count: 1, can_open: false, can_close: true, remaining: 0 }, 
            TestRun { token_index: 10, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 16, count: 1, can_open: false, can_close: true, remaining: 0 }, 
			TestRun { token_index: 20, count: 1, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 28, count: 3, can_open: true, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 4,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 5, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
            TestMatch { opener_index: 2, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 4, closer_index: 5, use_count: 1 },
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::EMPHASIS_ESCAPE_CAPTION_01",
    },
];

// The following are to test [`bh_cmark::parser::delimiter::DelimiterParser`] 
// implementations of Markdown's **can_open** (left‑flanking), and **can_close** 
// (right‑flanking) rules, where emphases can be neither. 
static LEFT_RIGHT_FLANKING_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_01,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 0, count: 2, can_open: true, can_close: false, remaining: 0 }, 
            TestRun { token_index: 3, count: 1, can_open: true, can_close: true, remaining: 0 }, 
            TestRun { token_index: 5, count: 1, can_open: true, can_close: true, remaining: 0 },
            TestRun { token_index: 7, count: 2, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 3, use_count: 2 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_01",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_02,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 1, count: 1, can_open: false, can_close: true, remaining: 1 }, 
            TestRun { token_index: 3, count: 1, can_open: true, can_close: false, remaining: 1 }, 
        ],
        expected_match_len: 0,
        expected_matches: &[],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_02",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_03,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 1, count: 1, can_open: true, can_close: true, remaining: 0 }, 
            TestRun { token_index: 3, count: 1, can_open: false, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 }
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_03",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_04,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 1, count: 1, can_open: true, can_close: true, remaining: 0 }, 
            TestRun { token_index: 3, count: 1, can_open: true, can_close: true, remaining: 0 }, 
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 }
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_04",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_05,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 0, count: 1, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 3, count: 1, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 5, count: 1, can_open: false, can_close: true, remaining: 0 },
            TestRun { token_index: 8, count: 1, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_05",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_06,
        expected_run_len: 3,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 4, count: 1, can_open: true, can_close: true, remaining: 0 },
            TestRun { token_index: 6, count: 3, can_open: false, can_close: true, remaining: 1 },
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 2, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_06",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_07,
        expected_run_len: 2,
        expected_runs: &[
            TestRun { token_index: 0, count: 1, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 2, count: 1, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 }
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_07",
    },
];

static MODULO_3_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: MODULO_3_01,
        expected_run_len: 3,
        expected_runs: &[
            TestRun { token_index: 0, count: 1, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 2, count: 2, can_open: true, can_close: true, remaining: 2 },
            TestRun { token_index: 5, count: 1, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 1,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 2, use_count: 1 }
        ],
        unique_tag: "MODULO_3_TESTS::MODULO_3_01",
    },
    TestInputAndResult {
        source: MODULO_3_02,
        expected_run_len: 3,
        expected_runs: &[
            TestRun { token_index: 0, count: 5, can_open: true, can_close: false, remaining: 2 },
            TestRun { token_index: 6, count: 1, can_open: true, can_close: true, remaining: 0 },
            TestRun { token_index: 8, count: 4, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 3,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 2, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 2, use_count: 1 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
        ],
        unique_tag: "MODULO_3_TESTS::MODULO_3_02",
    },
    TestInputAndResult {
        source: MODULO_3_03,
        expected_run_len: 3,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 4, count: 2, can_open: true, can_close: true, remaining: 0 },
            TestRun { token_index: 7, count: 1, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 2, use_count: 1 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
        ],
        unique_tag: "MODULO_3_TESTS::MODULO_3_03",
    },
];

static EDGE_CASE_INPUT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: EDGE_CASE_NESTING_01,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 2 },
            TestRun { token_index: 5, count: 2, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 8, count: 1, can_open: false, can_close: true, remaining: 0 },
            TestRun { token_index: 11, count: 2, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 3,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 1, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
        ],
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_01",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_02,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 1 },
            TestRun { token_index: 5, count: 1, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 7, count: 2, can_open: false, can_close: true, remaining: 0 },
            TestRun { token_index: 11, count: 1, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 3,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 3, use_count: 1 },
            TestMatch { opener_index: 0, closer_index: 2, use_count: 1 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
        ],
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_02",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_03,
        expected_run_len: 4,
        expected_runs: &[
            TestRun { token_index: 0, count: 2, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 4, count: 1, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 6, count: 1, can_open: false, can_close: true, remaining: 0 },
            TestRun { token_index: 9, count: 2, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 3, use_count: 2 },
            TestMatch { opener_index: 1, closer_index: 2, use_count: 1 },
        ],
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_03",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_04,
        expected_run_len: 3,
        expected_runs: &[
            TestRun { token_index: 0, count: 3, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 4, count: 1, can_open: false, can_close: true, remaining: 0 },
            TestRun { token_index: 7, count: 2, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 2,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 2, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_04",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_05,
        expected_run_len: 3,
        expected_runs: &[
            TestRun { token_index: 0, count: 4, can_open: true, can_close: false, remaining: 0 },
            TestRun { token_index: 5, count: 3, can_open: false, can_close: true, remaining: 0 },
            TestRun { token_index: 10, count: 1, can_open: false, can_close: true, remaining: 0 },
        ],
        expected_match_len: 3,
        expected_matches: &[
            TestMatch { opener_index: 0, closer_index: 2, use_count: 1 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 2 },
            TestMatch { opener_index: 0, closer_index: 1, use_count: 1 },
        ],
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_05",
    },
];

static TEST_SET: &[&[TestInputAndResult]] = &[
    PLAIN_TEXT_TESTS,
    HASH_ESCAPE_TESTS,
    ASTERISK_TESTS,
    NESTED_ASTERISK_TESTS,
    BUG_ASTERISK_TESTS,
    SPECIAL_ESCAPE_TESTS,
    EMPHASIS_UNEVEN_TOKEN_TESTS,
    HEADER_CAPTION_TEXT_TESTS,
    LEFT_RIGHT_FLANKING_TESTS,
    MODULO_3_TESTS,
    EDGE_CASE_INPUT_TESTS,
];

fn verify_runs(runs: &DelimiterRunVector,
    matches: &DelimiterMatchVector,
    result: &TestInputAndResult
) {
    assert_eq!(runs.len(), result.expected_run_len, "{} Total number of runs", result.unique_tag);
    assert_eq!(matches.len(), result.expected_match_len, "{} Total number of matches", result.unique_tag);

    for (index, run) in runs.iter().enumerate() {
        let expected_run = &result.expected_runs[index];

        assert_eq!(run.token_index(), expected_run.token_index,
            "{} token_index {}", result.unique_tag, index+1);

        assert_eq!(run.count(), expected_run.count,
            "{} count {}", result.unique_tag, index+1);

        assert_eq!(run.can_open(), expected_run.can_open,
            "{} can_open {}", result.unique_tag, index+1);

        assert_eq!(run.can_close(), expected_run.can_close,
            "{} can_close {}", result.unique_tag, index+1);

        assert_eq!(run.remaining(), expected_run.remaining,
            "{} remaining {}", result.unique_tag, index+1);
    }

    for (index, m) in matches.iter().enumerate() {
        let expected_match = &result.expected_matches[index];

        assert_eq!(m.opener_index(), expected_match.opener_index,
            "{} opener_index {}", result.unique_tag, index+1);

        assert_eq!(m.closer_index(), expected_match.closer_index,
            "{} closer_index {}", result.unique_tag, index+1);

        assert_eq!(m.use_count(), expected_match.use_count,
            "{} use_count {}", result.unique_tag, index+1);
    }
}

#[test]
fn test_delimiter_parser_match_delimiters() {
    for group in TEST_SET {
        for test in *group {
            let mut scanner = Scanner::new(test.source);
            let res = scanner.scan_tokens();

            assert!(res.is_ok(), "Scanning {} should be valid", test.unique_tag);

            let tokens = res.unwrap();
            let ignored_tokens: HashSet<usize> = HashSet::new();
            let mut parser = DelimiterParser::new(&tokens, &ignored_tokens);

            let res = parser.pre_process();
            assert!(res.is_ok(), "pre_process() {} should be valid", test.unique_tag);

            let res = parser.classify_can_open_close();
            assert!(res.is_ok(), "classify_can_open_close() {} should be valid", test.unique_tag);

            let res = parser.match_delimiters();
            assert!(res.is_ok(), "match_delimiters() {} should be valid", test.unique_tag);

            // println!("\n{:?}\n", parser.run_vector());
            // println!("\n{:?}\n", parser.match_vector());

            verify_runs(parser.run_vector(), parser.match_vector(), test);
        }
    }
}