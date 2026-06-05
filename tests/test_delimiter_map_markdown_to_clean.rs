/* 20/04/2026 */

//! Test [`bh_cmark::parser::delimiter::DelimiterRun`]'s 
//! `map_markdown_to_clean()` method.
//! 
//! Test only inline Markdown texts, not block elements.
//! 
//! To run test for this module only:
//!
//!     * cargo test --test test_delimiter_map_markdown_to_clean
//!
//! To run a specific test method:
//!
//!     * cargo test test_delimiter_map_markdown_to_clean -- --exact [--nocapture]
//!

use std::collections::HashSet;

use bh_cmark::scanner::Scanner;
use bh_cmark::parser::delimiter::{
    DelimiterParser,
};

mod common;
use common::test_constant::*;

#[derive(Debug)]
struct TestInputAndResult<'a> {
    source: &'a str,
    clean_text: &'a str,
    unique_tag: &'a str,
}

static PLAIN_TEXT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: HEADER_02_TEXT,
        clean_text: HEADER_02_TEXT,
        unique_tag: "PLAIN_TEXT_TESTS::HEADER_02_TEXT",
    },
    TestInputAndResult {
        source: TOKEN_LEXEME_AS_TEXT_01,
        clean_text: TOKEN_LEXEME_AS_TEXT_01,
        unique_tag: "PLAIN_TEXT_TESTS::TOKEN_LEXEME_AS_TEXT_01",
    },
];

static HASH_ESCAPE_TESTS: &'static [TestInputAndResult] = &[
    TestInputAndResult {
        source: HASH_ESCAPE_01,
        clean_text: HASH_ESCAPE_01_TEXT, 
        unique_tag: "HASH_ESCAPE_TESTS::HASH_ESCAPE_01",
    },
    TestInputAndResult {
        source: HASH_ESCAPE_02,
        clean_text: HASH_ESCAPE_02_TEXT, 
        unique_tag: "HASH_ESCAPE_TESTS::HASH_ESCAPE_02",
    },
];

static ASTERISK_TESTS: &'static [TestInputAndResult] = &[
    TestInputAndResult {
        source: ASTERISK_01,
        clean_text: ASTERISK_01_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_01",
    },
    TestInputAndResult {
        source: ASTERISK_02,
        clean_text: ASTERISK_02_PARSED_TEXT, 
        unique_tag: "ASTERISK_TESTS::ASTERISK_02",
    },
    TestInputAndResult {
        source: ASTERISK_03,
        clean_text: ASTERISK_03_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_03",
    },
    TestInputAndResult {
        source: ASTERISK_04,
        clean_text: ASTERISK_04_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_04",
    },
    TestInputAndResult {
        source: ASTERISK_05,
        clean_text: ASTERISK_05_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_05",
    },
    TestInputAndResult {
        source: ASTERISK_06,
        clean_text: ASTERISK_06_PARSED_TEXT, 
        unique_tag: "ASTERISK_TESTS::ASTERISK_06",
    },
    TestInputAndResult {
        source: ASTERISK_07,
        clean_text: ASTERISK_07_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_07",
    },
    TestInputAndResult {
        source: ASTERISK_08,
        clean_text: ASTERISK_08_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_08",
    },
    TestInputAndResult {
        source: ASTERISK_09,
        clean_text: ASTERISK_09_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_09",
    },
    TestInputAndResult {
        source: ASTERISK_10,
        clean_text: ASTERISK_10_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_10",
    },
    TestInputAndResult {
        source: ASTERISK_11,
        clean_text: ASTERISK_11_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_11",
    },
    TestInputAndResult {
        source: ASTERISK_12,
        clean_text: ASTERISK_12_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_12",
    },
    TestInputAndResult {
        source: ASTERISK_20,
        clean_text: ASTERISK_20_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_20",
    },
    TestInputAndResult {
        source: ASTERISK_21,
        clean_text: ASTERISK_21_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_21",
    },
    TestInputAndResult {
        source: ASTERISK_22,
        clean_text: ASTERISK_22_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_22",
    },
    TestInputAndResult {
        source: ASTERISK_23,
        clean_text: ASTERISK_23_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_23",
    },
    TestInputAndResult {
        source: ASTERISK_24,
        clean_text: ASTERISK_24_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_24",
    },
    TestInputAndResult {
        source: ASTERISK_25,
        clean_text: ASTERISK_25_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_25",
    },
    TestInputAndResult {
        source: ASTERISK_26,
        clean_text: ASTERISK_26_PARSED_TEXT,
        unique_tag: "ASTERISK_TESTS::ASTERISK_26",
    },
];

static NESTED_ASTERISK_TESTS: &'static [TestInputAndResult] = &[
    TestInputAndResult {
        source: ASTERISK_13,
        clean_text: ASTERISK_13_PARSED_TEXT,
        unique_tag: "NESTED_ASTERISK_TESTS::ASTERISK_13",
    },
    TestInputAndResult {
        source: ASTERISK_14,
        clean_text: ASTERISK_14_PARSED_TEXT,
        unique_tag: "NESTED_ASTERISK_TESTS::ASTERISK_14",
    },
    TestInputAndResult {
        source: EMPHASIS_BOLD_INSIDE_ITALIC_01,
        clean_text: EMPHASIS_BOLD_INSIDE_ITALIC_01_PARSED_TEXT,
        unique_tag: "NESTED_ASTERISK_TESTS::EMPHASIS_BOLD_INSIDE_ITALIC_01",
    },
    TestInputAndResult {
        source: EMPHASIS_BOLD_INSIDE_ITALIC_02,
        clean_text: EMPHASIS_BOLD_INSIDE_ITALIC_02_PARSED_TEXT,
        unique_tag: "NESTED_ASTERISK_TESTS::EMPHASIS_BOLD_INSIDE_ITALIC_02",
    },
];

// These are the bug fixed in https://github.com/behai-nguyen/polyglot_pdf/blob/main/pdf_06_text_styling/src/inline_parser.rs
// Addressed in this iteration. That is they produce the same output as VSC, and etc.
static BUG_ASTERISK_TESTS: &'static [TestInputAndResult] = &[
    TestInputAndResult {
        source: ASTERISK_15,
        clean_text: ASTERISK_15_PARSED_TEXT, // no scanned text.
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_15",
    },
    TestInputAndResult {
        source: ASTERISK_16,
        clean_text: ASTERISK_16_PARSED_TEXT,
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_16",
    },
    TestInputAndResult {
        source: ASTERISK_17,
        clean_text: ASTERISK_17_PARSED_TEXT,
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_17",
    },
    TestInputAndResult {
        source: ASTERISK_18,
        clean_text: ASTERISK_18_PARSED_TEXT,
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_18",
    },
    TestInputAndResult {
        source: ASTERISK_19,
        clean_text: ASTERISK_19_PARSED_TEXT,
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_19",
    },
];

static SPECIAL_ESCAPE_TESTS: &'static [TestInputAndResult] = &[
    TestInputAndResult {
        source: RECURRING_ESCAPE_01,
        clean_text: RECURRING_ESCAPE_01_TEXT,
        unique_tag: "SPECIAL_ESCAPE_TESTS::RECURRING_ESCAPE_01",
    },
    TestInputAndResult {
        source: ESCAPE_NEWLINE_01,
        clean_text: ESCAPE_NEWLINE_01_TEXT,
        unique_tag: "SPECIAL_ESCAPE_TESTS::ESCAPE_NEWLINE_01",
    },
    TestInputAndResult {
        source: ESCAPE_INSIDE_EMPHASIS_01,
        clean_text: ESCAPE_INSIDE_EMPHASIS_01_TEXT,
        unique_tag: "SPECIAL_ESCAPE_TESTS::ESCAPE_INSIDE_EMPHASIS_01",
    },
    TestInputAndResult {
        source: EMPHASIS_ADJACENT_ESCAPE_01,
        clean_text: EMPHASIS_ADJACENT_ESCAPE_01_TEXT,
        unique_tag: "SPECIAL_ESCAPE_TESTS::EMPHASIS_ADJACENT_ESCAPE_01",
    },
];

static EMPHASIS_UNEVEN_TOKEN_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_01,
        clean_text: EMPHASIS_UNEVEN_TOKEN_01_PARSED_TEXT,
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_01",
    },
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_02,
        clean_text: EMPHASIS_UNEVEN_TOKEN_02_PARSED_TEXT,
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_02",
    },
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_03,
        clean_text: EMPHASIS_UNEVEN_TOKEN_03_PARSED_TEXT,
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_03",
    },
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_04,
        clean_text: EMPHASIS_UNEVEN_TOKEN_04_PARSED_TEXT,
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_04",
    },
];


/// These are the captions of some image blocks defined above, these captions 
/// are special, such as: multilines, recurring escape, emphasis, etc. They 
/// are defined here to test the [`bh_cmark::parser::delimiter::DelimiterParser`] 
/// various methods.
static HEADER_CAPTION_TEXT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: MULTILINE_LINE_CAPTION_01,
        clean_text: MULTILINE_LINE_CAPTION_01,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTILINE_LINE_CAPTION_01",
    },
    TestInputAndResult {
        source: MULTILINE_LINE_CAPTION_02,
        clean_text: MULTILINE_LINE_CAPTION_02,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTILINE_LINE_CAPTION_02",
    },
    TestInputAndResult {
        source: MULTILINE_LINE_CAPTION_03,
        clean_text: MULTILINE_LINE_CAPTION_03,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTILINE_LINE_CAPTION_03",
    },
    TestInputAndResult {
        source: MULTI_LINGUAL_CAPTION_01,
        clean_text: MULTI_LINGUAL_CAPTION_01,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTI_LINGUAL_CAPTION_01",
    },
    TestInputAndResult {
        source: WIN_STYLE_PATH_TEXT,
        clean_text: WIN_STYLE_PATH_PARSED_TEXT,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::WIN_STYLE_PATH_TEXT",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_01,
        clean_text: ESCAPE_CAPTION_01_PARSED_TEXT,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_01",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_02,
        clean_text: ESCAPE_CAPTION_02_PARSED_TEXT,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_02",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_03,
        clean_text: ESCAPE_CAPTION_03,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_03",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_04,
        clean_text: ESCAPE_CAPTION_04_PARSED_TEXT,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_04",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_05,
        clean_text: ESCAPE_CAPTION_05_PARSED_TEXT,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_05",
    },
    TestInputAndResult {
        source: EMPHASIS_CAPTION_01,
        clean_text: EMPHASIS_CAPTION_01_PARSED_TEXT,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::EMPHASIS_CAPTION_01",
    },
    TestInputAndResult {
        source: NESTED_EMPHASIS_CAPTION_01,
        clean_text: NESTED_EMPHASIS_CAPTION_01_PARSED_TEXT,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::NESTED_EMPHASIS_CAPTION_01",
    },
    TestInputAndResult {
        source: EMPHASIS_ESCAPE_CAPTION_01,
        clean_text: EMPHASIS_ESCAPE_CAPTION_01_PARSED_TEXT,
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::EMPHASIS_ESCAPE_CAPTION_01",
    },
];

/// The following are to test [`bh_cmark::parser::delimiter::DelimiterParser`] 
/// implementations of Markdown's **can_open** (left‑flanking), and **can_close** 
/// (right‑flanking) rules, where emphases can be neither. 
static LEFT_RIGHT_FLANKING_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_01,
        clean_text: LEFT_RIGHT_FLANKING_01_PARSED_TEXT,
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_01",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_02,
        clean_text: LEFT_RIGHT_FLANKING_02,
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_02",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_03,
        clean_text: LEFT_RIGHT_FLANKING_03_PARSED_TEXT,
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_03",
    },    
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_04,
        clean_text: LEFT_RIGHT_FLANKING_04_PARSED_TEXT,
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_04",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_05,
        clean_text: LEFT_RIGHT_FLANKING_05_PARSED_TEXT,
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_05",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_06,
        clean_text: LEFT_RIGHT_FLANKING_06_PARSED_TEXT,
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_06",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_07,
        clean_text: LEFT_RIGHT_FLANKING_07_PARSED_TEXT,
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_07",
    },
];

static MODULO_3_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: MODULO_3_01,
        clean_text: MODULO_3_01_PARSED_TEXT,
        unique_tag: "MODULO_3_TESTS::MODULO_3_01",
    },
    TestInputAndResult {
        source: MODULO_3_02,
        clean_text: MODULO_3_02_PARSED_TEXT,
        unique_tag: "MODULO_3_TESTS::MODULO_3_02",
    },
    TestInputAndResult {
        source: MODULO_3_03,
        clean_text: MODULO_3_03_PARSED_TEXT,
        unique_tag: "MODULO_3_TESTS::MODULO_3_03",
    },
];

static EDGE_CASE_INPUT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: EDGE_CASE_NESTING_01,
        clean_text: EDGE_CASE_NESTING_01_PARSED_TEXT,
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_01",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_02,
        clean_text: EDGE_CASE_NESTING_02_PARSED_TEXT,
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_02",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_03,
        clean_text: EDGE_CASE_NESTING_03_PARSED_TEXT,
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_03",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_04,
        clean_text: EDGE_CASE_NESTING_04_PARSED_TEXT,
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_04",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_05,
        clean_text: EDGE_CASE_NESTING_05_PARSED_TEXT,
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

fn verify_runs(clean_text: &str, result: &TestInputAndResult
) {
    assert_eq!(result.clean_text, clean_text, "{} clean_text", result.unique_tag);
}

#[test]
fn test_delimiter_map_markdown_to_clean() {
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
            assert!(res.is_ok(), "match_delimiters() should be valid");

            let res = parser.map_markdown_to_clean();
            assert!(res.is_ok(), "map_markdown_to_clean() should be valid");

            verify_runs(&res.unwrap(), test);
        }
    }
}