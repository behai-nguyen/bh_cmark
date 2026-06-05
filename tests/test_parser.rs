/* 27/04/2026 */

//! Includes all tests from `test_scanner.rs`.
//! 
//! To run test for this module only:
//!
//!     * cargo test --test test_parser
//!
//! To run a specific test method:
//!
//!     * cargo test test_parser_parse -- --exact [--nocapture]
//!     * cargo test test_parser_parse_invalid -- --exact [--nocapture]
//!

use bh_cmark::ast::{
    AstBlock, 
    InlineContent, 
    SpanStyle
};
use bh_cmark::scanner::Scanner;
use bh_cmark::parser::parser::Parser;

mod common;
use common::test_constant::*;

#[cfg(test)]
#[derive(PartialEq, Eq, Debug)]
enum BlockType {
    Header,
    Paragraph,
    Image,
    Thematic,
    Code,
}

#[cfg(test)]
#[derive(Debug)]
struct TestSpan<'a> {
    pub start: usize,
    pub end: usize,    
    pub style: SpanStyle,
    pub text: &'a str,
}

#[cfg(test)]
#[derive(Debug)]
struct TestBlock<'a> {
    block_type: BlockType,
    /// Applicable only for `BlockType::Header`.
    header_level: u8,
    /// Applicable only for `BlockType::Image`.
    image_path: &'a str,
    /// Common fields.
    /// Clean text: `BlockType::Paragraph`'s text, 
    ///     `BlockType::Image`'s caption.
    ///     `BlockType::Code`'s content text.
    text: &'a str,
    /// Emphasis spans for `text`.
    spans: &'static [TestSpan<'static>],
    /// Code language. Applicable only for `BlockType::Code`.
    language: Option<String>,
}

#[derive(Debug)]
struct TestInputAndResult<'a> {
    source: &'a str,
    expected_len: usize,
    expected_blocks: &'static [TestBlock<'static>],
    unique_tag: &'a str,
}

#[derive(Debug)]
struct TestInvalidInputAndResult<'a> {
    result: TestInputAndResult<'a>,
    error_messages: &'static [&'static str],
}

static PLAIN_TEXT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: HEADER_02_TEXT,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: HEADER_02_TEXT, spans: &[], language: None},
        ],
        unique_tag: "PLAIN_TEXT_TESTS::HEADER_02_TEXT",
    },
    TestInputAndResult {
        source: TOKEN_LEXEME_AS_TEXT_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: TOKEN_LEXEME_AS_TEXT_01, spans: &[], language: None},
        ],
        unique_tag: "PLAIN_TEXT_TESTS::TOKEN_LEXEME_AS_TEXT_01",
    },
];

static HEADER_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: HEADER_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Header, header_level: 1, image_path: "", 
                text: HEADER_01_TEXT, spans: &[], language: None},
        ],
        unique_tag: "HEADER_TESTS::HEADER_01",
    },
    TestInputAndResult {
        source: HEADER_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Header, header_level: 2, image_path: "", 
                text: HEADER_02_TEXT, spans: &[], language: None},
        ],
        unique_tag: "HEADER_TESTS::HEADER_02",
    },
    TestInputAndResult {
        source: HEADER_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Header, header_level: 1, image_path: "", 
                text: HEADER_03_TEXT, spans: &[
                    TestSpan { start: 0, end: 121, style: SpanStyle::Bold, text: HEADER_03_TEXT },
                    TestSpan { start: 18, end: 42, style: SpanStyle::Italic, text: "Fontainebleau 14/09/1946" },
                    TestSpan { start: 84, end: 96, style: SpanStyle::Italic, text: "tiêu diệt" },
                ], language: None},                
        ],
        unique_tag: "HEADER_TESTS::HEADER_03",
    },
    TestInputAndResult {
        source: HEADER_04,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Header, header_level: 1, image_path: "", 
                text: HEADER_04_TEXT, spans: &[
                    TestSpan { start: 0, end: 121, style: SpanStyle::Bold, text: HEADER_03_TEXT },
                    TestSpan { start: 18, end: 42, style: SpanStyle::Italic, text: "Fontainebleau 14/09/1946" },
                    TestSpan { start: 84, end: 96, style: SpanStyle::Italic, text: "tiêu diệt" },
                ], language: None},
        ],
        unique_tag: "HEADER_TESTS::HEADER_04",
    },
    TestInputAndResult {
        source: HEADER_05,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Header, header_level: 4, image_path: "", 
                text: "", spans: &[], language: None},
        ],
        unique_tag: "HEADER_TESTS::HEADER_05",
    },
    TestInputAndResult {
        source: HEADER_06,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Header, header_level: 3, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "HEADER_TESTS::HEADER_06",
    },
];

static HEADER_EXAMPLE_70_VARIANT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: HEADER_EXAMPLE_70_VARIANT_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: HEADER_EXAMPLE_70_VARIANT_01_PARSED_TEXT, 
                spans: &[
                    TestSpan { start: 4, end: 9, style: SpanStyle::Italic, text: "# bar" },
                ], language: None},
        ],
        unique_tag: "HEADER_EXAMPLE_70_VARIANT_TESTS::HEADER_EXAMPLE_70_VARIANT_01",
    },
    TestInputAndResult {
        source: HEADER_EXAMPLE_70_VARIANT_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: HEADER_EXAMPLE_70_VARIANT_02_PARSED_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "HEADER_EXAMPLE_70_VARIANT_TESTS::HEADER_EXAMPLE_70_VARIANT_02",
    },
    TestInputAndResult {
        source: HEADER_EXAMPLE_70_VARIANT_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: HEADER_EXAMPLE_70_VARIANT_03, spans: &[], language: None},
        ],
        unique_tag: "HEADER_EXAMPLE_70_VARIANT_TESTS::HEADER_EXAMPLE_70_VARIANT_03",
    },
];

static HASH_ESCAPE_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: HASH_ESCAPE_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: HASH_ESCAPE_01_TEXT, spans: &[], language: None},
        ],
        unique_tag: "HASH_ESCAPE_TESTS::HASH_ESCAPE_01",
    },
    TestInputAndResult {
        source: HASH_ESCAPE_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: HASH_ESCAPE_02_TEXT, spans: &[], language: None},
        ],
        unique_tag: "HASH_ESCAPE_TESTS::HASH_ESCAPE_02",
    },
    TestInputAndResult {
        source: HEADER_HASH_ESCAPE_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Header, header_level: 1, image_path: "", 
                text: HEADER_HASH_ESCAPE_01_TEXT, spans: &[], language: None},
        ],
        unique_tag: "HASH_ESCAPE_TESTS::HEADER_HASH_ESCAPE_01",
    },
];

static ASTERISK_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: ASTERISK_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_01_PARSED_TEXT, spans: &[
                    TestSpan { start: 4, end: 24, style: SpanStyle::Bold, text: "Tưởng Vĩnh Kính" },
                    TestSpan { start: 47, end: 59, style: SpanStyle::Italic, text: "Trung Quốc" },
                    TestSpan { start: 87, end: 96, style: SpanStyle::Bold, text: "trang 339" },
                    TestSpan { start: 87, end: 96, style: SpanStyle::Italic, text: "trang 339" },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_01",
    },
    TestInputAndResult {
        source: ASTERISK_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_02_PARSED_TEXT, spans: &[
                    TestSpan { start: 4, end: 24, style: SpanStyle::Bold, text: "Tưởng Vĩnh Kính" },
                    TestSpan { start: 47, end: 59, style: SpanStyle::Italic, text: "Trung Quốc" },
                    TestSpan { start: 87, end: 96, style: SpanStyle::Bold, text: "trang 339" },
                    TestSpan { start: 87, end: 96, style: SpanStyle::Italic, text: "trang 339" },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_02",
    },
    TestInputAndResult {
        source: ASTERISK_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_03_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 20, style: SpanStyle::Bold, text: ASTERISK_03_SCANNED_TEXT },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_03",
    },
    TestInputAndResult {
        source: ASTERISK_04,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_04_PARSED_TEXT, spans: &[
                    TestSpan { start: 1, end: 21, style: SpanStyle::Bold, text: ASTERISK_04_SCANNED_TEXT },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_04",
    },
    TestInputAndResult {
        source: ASTERISK_05,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_05_PARSED_TEXT, spans: &[
                    TestSpan { start: 2, end: 22, style: SpanStyle::Italic, text: ASTERISK_05_SCANNED_TEXT },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_05",
    },
    TestInputAndResult {
        source: ASTERISK_06,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_06_PARSED_TEXT, spans: &[], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_06",
    },
    TestInputAndResult {
        source: ASTERISK_07,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_07_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Bold, text: "bold" },
                    TestSpan { start: 0, end: 4, style: SpanStyle::Italic, text: "bold" },
                    TestSpan { start: 8, end: 12, style: SpanStyle::Bold, text: "more" },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_07",
    },
    TestInputAndResult {
        source: ASTERISK_08,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_08_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Bold, text: "bold" },
                    TestSpan { start: 0, end: 4, style: SpanStyle::Italic, text: "bold" },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_08",
    },
    TestInputAndResult {
        source: ASTERISK_09,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_09_PARSED_TEXT, spans: &[], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_09",
    },
    TestInputAndResult {
        source: ASTERISK_10,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_10_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 13, style: SpanStyle::Bold, text: ASTERISK_10_PARSED_TEXT },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_10",
    },
    TestInputAndResult {
        source: ASTERISK_11,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_11_PARSED_TEXT, spans: &[], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_11",
    },
    TestInputAndResult {
        source: ASTERISK_12,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_12_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 23, style: SpanStyle::Bold, text: ASTERISK_12_PARSED_TEXT },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_12",
    },
    TestInputAndResult {
        source: ASTERISK_20,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_20_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Bold, text: "bold" },
                    TestSpan { start: 0, end: 4, style: SpanStyle::Italic, text: "bold" },
                    TestSpan { start: 10, end: 14, style: SpanStyle::Bold, text: "more" },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_20",
    },
    TestInputAndResult {
        source: ASTERISK_21,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_21_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Bold, text: "bold" },
                    TestSpan { start: 0, end: 4, style: SpanStyle::Italic, text: "bold" },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_21",
    },
    TestInputAndResult {
        source: ASTERISK_22,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_22_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Bold, text: "bold" },
                    TestSpan { start: 0, end: 4, style: SpanStyle::Italic, text: "bold" },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_22",
    },
    TestInputAndResult {
        source: ASTERISK_23,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_23_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Bold, text: "bold" },
                    TestSpan { start: 0, end: 4, style: SpanStyle::Italic, text: "bold" },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_23",
    },
    TestInputAndResult {
        source: ASTERISK_24,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_24_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Bold, text: "bold" },
                    TestSpan { start: 0, end: 4, style: SpanStyle::Italic, text: "bold" },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_24",
    },
    TestInputAndResult {
        source: ASTERISK_25,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", 
                text: ASTERISK_25_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Bold, text: "bold" },
                    TestSpan { start: 0, end: 4, style: SpanStyle::Italic, text: "bold" },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_25",
    },
    TestInputAndResult {
        source: ASTERISK_26,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_26_PARSED_TEXT, spans: &[
                    TestSpan { start: 1, end: 21, style: SpanStyle::Bold, text: ASTERISK_26_SCANNED_TEXT },
                    TestSpan { start: 1, end: 21, style: SpanStyle::Bold, text: ASTERISK_26_SCANNED_TEXT },
                ], language: None},
        ],
        unique_tag: "ASTERISK_TESTS::ASTERISK_26",
    },
];

static NESTED_ASTERISK_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: ASTERISK_13,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_13_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 28, style: SpanStyle::Bold, text: ASTERISK_13_PARSED_TEXT },
                    TestSpan { start: 5, end: 23, style: SpanStyle::Italic, text: "italic inside bold" },
                ], language: None},
        ],
        unique_tag: "NESTED_ASTERISK_TESTS::ASTERISK_13",
    },
    TestInputAndResult {
        source: ASTERISK_14,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_14_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 69, style: SpanStyle::Bold, text: ASTERISK_14_PARSED_TEXT },
                    TestSpan { start: 14, end: 18, style: SpanStyle::Italic, text: "sử" },
                    TestSpan { start: 56, end: 68, style: SpanStyle::Italic, text: "chính trị" },
                ], language: None},
        ],
        unique_tag: "NESTED_ASTERISK_TESTS::ASTERISK_14",
    },
    TestInputAndResult {
        source: EMPHASIS_BOLD_INSIDE_ITALIC_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", 
                text: EMPHASIS_BOLD_INSIDE_ITALIC_01_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 24, style: SpanStyle::Italic, text: EMPHASIS_BOLD_INSIDE_ITALIC_01_PARSED_TEXT },
                    TestSpan { start: 3, end: 18, style: SpanStyle::Bold, text: "Sir John Seeley" },
                ], language: None},
        ],
        unique_tag: "NESTED_ASTERISK_TESTS::EMPHASIS_BOLD_INSIDE_ITALIC_01",
    },
    TestInputAndResult {
        source: EMPHASIS_BOLD_INSIDE_ITALIC_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: EMPHASIS_BOLD_INSIDE_ITALIC_02_PARSED_TEXT, spans: &[
                    TestSpan { start: 8, end: 32, style: SpanStyle::Italic, text: "-- Sir John Seeley, 1885" },
                    TestSpan { start: 11, end: 26, style: SpanStyle::Bold, text: "Sir John Seeley" },
                ], language: None},
        ],
        unique_tag: "NESTED_ASTERISK_TESTS::EMPHASIS_BOLD_INSIDE_ITALIC_02",
    },
];

// These are the bug fixed in https://github.com/behai-nguyen/polyglot_pdf/blob/main/pdf_06_text_styling/src/inline_parser.rs
// Addressed in this iteration. That is they produce the same output as VSC, and etc.
static BUG_ASTERISK_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: ASTERISK_15,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_15_PARSED_TEXT, spans: &[
                    TestSpan { start: 3, end: 51, style: SpanStyle::Bold, text: "Chính Ðạo, Việt Nam Niên Biểu, Tập 1A" },
                    TestSpan { start: 18, end: 41, style: SpanStyle::Italic, text: "Việt Nam Niên Biểu" },
                    TestSpan { start: 43, end: 51, style: SpanStyle::Italic, text: "Tập 1A" },
                ], language: None},
        ],
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_15",
    },
    TestInputAndResult {
        source: ASTERISK_16,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", 
                text: ASTERISK_16_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Bold, text: "bold" },
                    TestSpan { start: 0, end: 4, style: SpanStyle::Italic, text: "bold" },
                ], language: None},
        ],
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_16",
    },
    TestInputAndResult {
        source: ASTERISK_17,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_17_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 10, style: SpanStyle::Bold, text: ASTERISK_17_PARSED_TEXT },
                    TestSpan { start: 4, end: 6, style: SpanStyle::Italic, text: "bc" },
                    TestSpan { start: 8, end: 10, style: SpanStyle::Italic, text: "de" },
                ], language: None},
        ],
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_17",
    },
    TestInputAndResult {
        source: ASTERISK_18,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", 
                text: ASTERISK_18_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Bold, text: "xy z" },
                    TestSpan { start: 0, end: 2, style: SpanStyle::Italic, text: "xy" },
                ], language: None},
        ],
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_18",
    },
    TestInputAndResult {
        source: ASTERISK_19,
        expected_len: 1,
        expected_blocks: &[            
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: ASTERISK_19_PARSED_TEXT, spans: &[
                    TestSpan { start: 3, end: 23, style: SpanStyle::Bold, text: "Tưởng Vĩnh Kính" },
                ], language: None},
        ],
        unique_tag: "BUG_ASTERISK_TESTS::ASTERISK_19",
    },
];

static SPECIAL_ESCAPE_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: RECURRING_ESCAPE_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: RECURRING_ESCAPE_01_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "SPECIAL_ESCAPE_TESTS::RECURRING_ESCAPE_01",
    },
    TestInputAndResult {
        source: RECURRING_ESCAPE_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Header, header_level: 1, image_path: "", 
                text: RECURRING_ESCAPE_02_TEXT, spans: &[], language: None},
        ],
        unique_tag: "SPECIAL_ESCAPE_TESTS::RECURRING_ESCAPE_02",
    },
    TestInputAndResult {
        source: ESCAPE_NEWLINE_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: ESCAPE_NEWLINE_01_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "SPECIAL_ESCAPE_TESTS::ESCAPE_NEWLINE_01",
    },
    TestInputAndResult {
        source: ESCAPE_INSIDE_EMPHASIS_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: ESCAPE_INSIDE_EMPHASIS_01_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "SPECIAL_ESCAPE_TESTS::ESCAPE_INSIDE_EMPHASIS_01",
    },
    TestInputAndResult {
        source: EMPHASIS_ADJACENT_ESCAPE_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", 
                text: EMPHASIS_ADJACENT_ESCAPE_01_TEXT, spans: &[
                    TestSpan { start: 0, end: 6, style: SpanStyle::Italic, text: EMPHASIS_ADJACENT_ESCAPE_01_TEXT },
                ], language: None},
        ],
        unique_tag: "SPECIAL_ESCAPE_TESTS::EMPHASIS_ADJACENT_ESCAPE_01",
    },
];

 static EMPHASIS_UNEVEN_TOKEN_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: EMPHASIS_UNEVEN_TOKEN_01_PARSED_TEXT, 
                spans: &[
                    TestSpan { start: 1, end: 34, style: SpanStyle::Bold, text: "Đây Là Chú Thích Của Hình" },
                ], language: None},
        ],
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_01",
    },
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: EMPHASIS_UNEVEN_TOKEN_02_PARSED_TEXT, 
                spans: &[
                    TestSpan { start: 0, end: 33, style: SpanStyle::Bold, text: "Đây Là Chú Thích Của Hình" },
                ], language: None},
        ],
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_02",
    },
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: EMPHASIS_UNEVEN_TOKEN_03_PARSED_TEXT, 
                spans: &[
                    TestSpan { start: 1, end: 34, style: SpanStyle::Bold, text: "Đây Là Chú Thích Của Hình" },
                ], language: None},
        ],
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_03",
    },
    TestInputAndResult {
        source: EMPHASIS_UNEVEN_TOKEN_04,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: EMPHASIS_UNEVEN_TOKEN_04_PARSED_TEXT, 
                spans: &[
                    TestSpan { start: 0, end: 33, style: SpanStyle::Bold, text: "Đây Là Chú Thích Của Hình" },
                ], language: None},
        ],
        unique_tag: "EMPHASIS_UNEVEN_TOKEN_TESTS::EMPHASIS_UNEVEN_TOKEN_04",
    },
];

static IMAGE_BLOCK_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: IMAGE_BLOCK_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_01_PATH, text: IMAGE_BLOCK_01_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_TESTS::IMAGE_BLOCK_01",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_02_PATH, text: IMAGE_BLOCK_02_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_TESTS::IMAGE_BLOCK_02",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_03_PATH, text: IMAGE_BLOCK_03_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_TESTS::IMAGE_BLOCK_03",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_04,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_04_PATH, text: IMAGE_BLOCK_04_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_TESTS::IMAGE_BLOCK_04",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_MULTILINE_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_MULTILINE_01_PATH, 
                text: IMAGE_BLOCK_MULTILINE_01_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_TESTS::IMAGE_BLOCK_MULTILINE_01",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_MULTILINE_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_MULTILINE_02_PATH, 
                text: IMAGE_BLOCK_MULTILINE_02_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_TESTS::IMAGE_BLOCK_MULTILINE_02",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_MULTILINE_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_MULTILINE_03_PATH, 
                text: IMAGE_BLOCK_MULTILINE_03_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_TESTS::IMAGE_BLOCK_MULTILINE_03",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_MULTI_LINGUAL,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_MULTI_LINGUAL_PATH, 
                text: IMAGE_BLOCK_MULTI_LINGUAL_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_TESTS::IMAGE_BLOCK_MULTI_LINGUAL",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_05,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_05_PATH, text: IMAGE_BLOCK_05_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_TESTS::IMAGE_BLOCK_05",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_06,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_06_PATH, text: IMAGE_BLOCK_06_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_TESTS::IMAGE_BLOCK_06",
    },
];

static IMAGE_BLOCK_ESCAPE_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: IMAGE_BLOCK_WIN_STYLE,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_WIN_STYLE_PATH, 
                text: IMAGE_BLOCK_WIN_STYLE_CAPTION, spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_ESCAPE_TESTS::IMAGE_BLOCK_WIN_STYLE",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_ESCAPE_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_ESCAPE_01_PATH, 
                text: IMAGE_BLOCK_ESCAPE_01_CAPTION, spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_ESCAPE_TESTS::IMAGE_BLOCK_ESCAPE_01",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_ESCAPE_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_ESCAPE_02_PATH, 
                text: IMAGE_BLOCK_ESCAPE_02_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_ESCAPE_TESTS::IMAGE_BLOCK_ESCAPE_02",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_ESCAPE_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_ESCAPE_03_PATH, 
                text: IMAGE_BLOCK_ESCAPE_03_CAPTION, spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_ESCAPE_TESTS::IMAGE_BLOCK_ESCAPE_03",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_ESCAPE_04,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_ESCAPE_04_PATH, 
                text: IMAGE_BLOCK_ESCAPE_04_CAPTION, spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_ESCAPE_TESTS::IMAGE_BLOCK_ESCAPE_04",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_ESCAPE_05,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_ESCAPE_05_PATH, 
                text: IMAGE_BLOCK_ESCAPE_05_CAPTION, spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_ESCAPE_TESTS::IMAGE_BLOCK_ESCAPE_05",
    },
];

static IMAGE_BLOCK_EMPHASIS_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: IMAGE_BLOCK_EMPHASIS_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_EMPHASIS_01_PATH, 
                text: IMAGE_BLOCK_EMPHASIS_01_CAPTION, spans: &[
                    TestSpan { start: 0, end: 59, style: SpanStyle::Bold, text: IMAGE_BLOCK_EMPHASIS_01_CAPTION },
                    TestSpan { start: 0, end: 10, style: SpanStyle::Italic, text: "Mount Fuji" },
                    TestSpan { start: 13, end: 42, style: SpanStyle::Italic, text: "富士山, ふじさ, Fujisan" },
                    TestSpan { start: 45, end: 59, style: SpanStyle::Italic, text: "Núi Phú Sỹ" },
                ], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_EMPHASIS_TESTS::IMAGE_BLOCK_EMPHASIS_01",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_NESTED_EMPHASIS_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_NESTED_EMPHASIS_01_PATH, 
                text: IMAGE_BLOCK_NESTED_EMPHASIS_01_CAPTION, spans: &[
                    TestSpan { start: 0, end: 5, style: SpanStyle::Bold, text: IMAGE_BLOCK_NESTED_EMPHASIS_01_CAPTION },
                    TestSpan { start: 0, end: 5, style: SpanStyle::Italic, text: IMAGE_BLOCK_NESTED_EMPHASIS_01_CAPTION },
                    TestSpan { start: 2, end: 3, style: SpanStyle::Italic, text: "b" },
                ], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_EMPHASIS_TESTS::IMAGE_BLOCK_NESTED_EMPHASIS_01",
    },
];

static IMAGE_BLOCK_EMPHASIS_ESCAPE_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: IMAGE_BLOCK_EMPHASIS_ESCAPE_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_EMPHASIS_ESCAPE_01_PATH, 
                text: IMAGE_BLOCK_EMPHASIS_ESCAPE_01_CAPTION, spans: &[
                    TestSpan { start: 0, end: 61, style: SpanStyle::Bold, text: IMAGE_BLOCK_EMPHASIS_ESCAPE_01_CAPTION },
                    TestSpan { start: 0, end: 10, style: SpanStyle::Italic, text: "Mount Fuji" },
                    TestSpan { start: 13, end: 42, style: SpanStyle::Italic, text: "富士山, ふじさ, Fujisan" },
                    TestSpan { start: 45, end: 61, style: SpanStyle::Italic, text: "(Núi Phú Sỹ)" },
                ], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_EMPHASIS_ESCAPE_TESTS::IMAGE_BLOCK_EMPHASIS_ESCAPE_01",
    },
];

static IMAGE_BLOCK_MULTI_ITEMS_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: IMAGE_BLOCK_MULTI_ITEMS_01,
        expected_len: 2,
        // Note: `BlockType::Image` and `BlockType::Paragraph`.
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_MULTI_ITEMS_01_PATH_01, 
                text: IMAGE_BLOCK_MULTI_ITEMS_01_CAPTION_01, 
                spans: &[], language: None},
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: "![Á Ừ Ứ](./img/test2.png)", 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_MULTI_ITEMS_TESTS::IMAGE_BLOCK_MULTI_ITEMS_01",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_MULTI_ITEMS_02,
        expected_len: 2,
        // Note: `BlockType::Image` and `BlockType::Image`.
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_MULTI_ITEMS_02_PATH_01, 
                text: IMAGE_BLOCK_MULTI_ITEMS_02_CAPTION_01, 
                spans: &[], language: None},
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_MULTI_ITEMS_02_PATH_02, 
                text: IMAGE_BLOCK_MULTI_ITEMS_02_CAPTION_02, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_MULTI_ITEMS_TESTS::IMAGE_BLOCK_MULTI_ITEMS_02",
    },
];

static IMAGE_BLOCK_ADJACENT_TEXT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: IMAGE_BLOCK_ADJACENT_TEXT_01,
        expected_len: 2,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: "x", text: "a", spans: &[], language: None},
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: "text", spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_ADJACENT_TEXT_TESTS::IMAGE_BLOCK_ADJACENT_TEXT_01",
    },
];

// Space in path.
// Nested "[", "]", "(", and ")" in caption and path.
static IMAGE_BLOCK_SPACE_NESTED_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: IMAGE_BLOCK_SPACE_IN_PATH,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_SPACE_IN_PATH_PATH, 
                text: IMAGE_BLOCK_SPACE_IN_PATH_CAPTION, spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_SPACE_NESTED_TESTS::IMAGE_BLOCK_SPACE_IN_PATH",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_PARENS_IN_PATH_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_PARENS_IN_PATH_01_PATH, 
                text: IMAGE_BLOCK_PARENS_IN_PATH_01_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_SPACE_NESTED_TESTS::IMAGE_BLOCK_PARENS_IN_PATH_01",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_PARENS_IN_PATH_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_PARENS_IN_PATH_02_PATH, 
                text: IMAGE_BLOCK_PARENS_IN_PATH_02_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_SPACE_NESTED_TESTS::IMAGE_BLOCK_PARENS_IN_PATH_02",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_BRACKETS_IN_CAPTION_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_BRACKETS_IN_CAPTION_01_PATH, 
                text: IMAGE_BLOCK_BRACKETS_IN_CAPTION_01_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_SPACE_NESTED_TESTS::IMAGE_BLOCK_BRACKETS_IN_CAPTION_01",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_BRACKETS_IN_CAPTION_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_BRACKETS_IN_CAPTION_02_PATH, 
                text: IMAGE_BLOCK_BRACKETS_IN_CAPTION_02_CAPTION, 
                spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_SPACE_NESTED_TESTS::IMAGE_BLOCK_BRACKETS_IN_CAPTION_02",
    },

    TestInputAndResult {
        source: IMAGE_BLOCK_BRACKETS_IN_CAPTION_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_BRACKETS_IN_CAPTION_03_PATH, 
                text: IMAGE_BLOCK_BRACKETS_IN_CAPTION_03_CAPTION, spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_SPACE_NESTED_TESTS::IMAGE_BLOCK_BRACKETS_IN_CAPTION_03",
    },
    TestInputAndResult {
        source: IMAGE_BLOCK_MIXED_BRACKETS_PARENS_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Image, header_level: 0, 
                image_path: IMAGE_BLOCK_ESCAPE_01_PATH, 
                text: IMAGE_BLOCK_ESCAPE_01_CAPTION, spans: &[], language: None},
        ],
        unique_tag: "IMAGE_BLOCK_SPACE_NESTED_TESTS::IMAGE_BLOCK_MIXED_BRACKETS_PARENS_01",
    },
];

/// These are the captions of some image blocks defined above, these captions 
/// are special, such as: multilines, recurring escape, emphasis, etc. They 
/// are defined here to test the [`bh_cmark::parser::delimiter::DelimiterParser`] 
/// various methods. This adds little value to this suite of test, they are included 
/// for the purpose of completeness.
static HEADER_CAPTION_TEXT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: MULTILINE_LINE_CAPTION_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: MULTILINE_LINE_CAPTION_01, 
                spans: &[], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTILINE_LINE_CAPTION_01",
    },
    TestInputAndResult {
        source: MULTILINE_LINE_CAPTION_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: MULTILINE_LINE_CAPTION_02, 
                spans: &[], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTILINE_LINE_CAPTION_02",
    },
    TestInputAndResult {
        source: MULTILINE_LINE_CAPTION_03,
        expected_len: 0,
        expected_blocks: &[],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTILINE_LINE_CAPTION_03",
    },
    TestInputAndResult {
        source: MULTI_LINGUAL_CAPTION_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: MULTI_LINGUAL_CAPTION_01, 
                spans: &[], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::MULTI_LINGUAL_CAPTION_01",
    },
    TestInputAndResult {
        source: WIN_STYLE_PATH_TEXT,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: WIN_STYLE_PATH_PARSED_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::WIN_STYLE_PATH_TEXT",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: ESCAPE_CAPTION_01_PARSED_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_01",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: ESCAPE_CAPTION_02_PARSED_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_02",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: ESCAPE_CAPTION_03, spans: &[], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_03",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_04,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: ESCAPE_CAPTION_04_PARSED_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_04",
    },
    TestInputAndResult {
        source: ESCAPE_CAPTION_05,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: ESCAPE_CAPTION_05_PARSED_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::ESCAPE_CAPTION_05",
    },
    TestInputAndResult {
        source: EMPHASIS_CAPTION_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: EMPHASIS_CAPTION_01_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 59, style: SpanStyle::Bold, text: EMPHASIS_CAPTION_01_PARSED_TEXT },
                    TestSpan { start: 0, end: 10, style: SpanStyle::Italic, text: "Mount Fuji" },
                    TestSpan { start: 13, end: 42, style: SpanStyle::Italic, text: "富士山, ふじさ, Fujisan" },
                    TestSpan { start: 45, end: 59, style: SpanStyle::Italic, text: "Núi Phú Sỹ" },
                ], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::EMPHASIS_CAPTION_01",
    },
    TestInputAndResult {
        source: NESTED_EMPHASIS_CAPTION_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: NESTED_EMPHASIS_CAPTION_01_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 5, style: SpanStyle::Bold, text: NESTED_EMPHASIS_CAPTION_01_PARSED_TEXT },
                    TestSpan { start: 0, end: 5, style: SpanStyle::Italic, text: NESTED_EMPHASIS_CAPTION_01_PARSED_TEXT },
                    TestSpan { start: 2, end: 3, style: SpanStyle::Italic, text: "b" },
                ], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::NESTED_EMPHASIS_CAPTION_01",
    },
    TestInputAndResult {
        source: EMPHASIS_ESCAPE_CAPTION_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: EMPHASIS_ESCAPE_CAPTION_01_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 61, style: SpanStyle::Bold, text: EMPHASIS_ESCAPE_CAPTION_01_PARSED_TEXT },
                    TestSpan { start: 0, end: 10, style: SpanStyle::Italic, text: "Mount Fuji" },
                    TestSpan { start: 13, end: 42, style: SpanStyle::Italic, text: "富士山, ふじさ, Fujisan" },
                    TestSpan { start: 45, end: 61, style: SpanStyle::Italic, text: "(Núi Phú Sỹ)" },                    
                ], language: None},
        ],
        unique_tag: "HEADER_CAPTION_TEXT_TESTS::EMPHASIS_ESCAPE_CAPTION_01",
    },
];


/// The following are to test [`bh_cmark::parser::delimiter::DelimiterParser`] 
/// implementations of Markdown's **can_open** (left‑flanking), and **can_close** 
/// (right‑flanking) rules, where emphases can be neither. 
/// This adds little value to this suite of test, they are included for the purpose 
/// of completeness.
static LEFT_RIGHT_FLANKING_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: LEFT_RIGHT_FLANKING_01_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 14, style: SpanStyle::Bold, text: LEFT_RIGHT_FLANKING_01_PARSED_TEXT },
                    TestSpan { start: 3, end: 9, style: SpanStyle::Italic, text: "Đại" },
                ], language: None},
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_01",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: LEFT_RIGHT_FLANKING_02, spans: &[], language: None},
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_02",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: LEFT_RIGHT_FLANKING_03_PARSED_TEXT, spans: &[
                    TestSpan { start: 3, end: 6, style: SpanStyle::Italic, text: "bar" },
                ], language: None},
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_03",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_04,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: LEFT_RIGHT_FLANKING_04_PARSED_TEXT, spans: &[
                    TestSpan { start: 3, end: 6, style: SpanStyle::Italic, text: "bar" },
                ], language: None},
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_04",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_05,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: LEFT_RIGHT_FLANKING_05_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 11, style: SpanStyle::Italic, text: LEFT_RIGHT_FLANKING_05_PARSED_TEXT },
                    TestSpan { start: 4, end: 7, style: SpanStyle::Italic, text: "bar" },
                ], language: None},
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_05",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_06,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: LEFT_RIGHT_FLANKING_06_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 2, style: SpanStyle::Bold, text: "ab" },
                    TestSpan { start: 0, end: 1, style: SpanStyle::Italic, text: "a" },
                ], language: None},
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_06",
    },
    TestInputAndResult {
        source: LEFT_RIGHT_FLANKING_07,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: LEFT_RIGHT_FLANKING_07_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 3, style: SpanStyle::Italic, text: LEFT_RIGHT_FLANKING_07_PARSED_TEXT },
                ], language: None},
        ],
        unique_tag: "LEFT_RIGHT_FLANKING_TESTS::LEFT_RIGHT_FLANKING_07",
    },
];

static MODULO_3_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: MODULO_3_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: MODULO_3_01_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 4, style: SpanStyle::Italic, text: MODULO_3_01_PARSED_TEXT },
                ], language: None},
        ],
        unique_tag: "MODULO_3_TESTS::MODULO_3_01",
    },
    TestInputAndResult {
        source: MODULO_3_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: MODULO_3_02_PARSED_TEXT, spans: &[
                    TestSpan { start: 2, end: 12, style: SpanStyle::Bold, text: "Helloworld" },
                    TestSpan { start: 2, end: 12, style: SpanStyle::Italic, text: "Helloworld" },
                    TestSpan { start: 7, end: 12, style: SpanStyle::Italic, text: "world" },
                ], language: None},
        ],
        unique_tag: "MODULO_3_TESTS::MODULO_3_02",
    },
    TestInputAndResult {
        source: MODULO_3_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, image_path: "", 
                text: MODULO_3_03_PARSED_TEXT, spans: &[
                    TestSpan { start: 0, end: 2, style: SpanStyle::Italic, text: MODULO_3_03_PARSED_TEXT },
                    TestSpan { start: 0, end: 1, style: SpanStyle::Bold, text: "a" },                    
                ], language: None},
        ],
        unique_tag: "MODULO_3_TESTS::MODULO_3_03",
    },
];

static EDGE_CASE_INPUT_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: EDGE_CASE_NESTING_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: EDGE_CASE_NESTING_01_PARSED_TEXT, 
                spans: &[
                    TestSpan { start: 2, end: 7, style: SpanStyle::Italic, text: "a b c" }, 
                    TestSpan { start: 4, end: 7, style: SpanStyle::Italic, text: "b c" },
                    TestSpan { start: 4, end: 5, style: SpanStyle::Italic, text: "b" },
                ], language: None},
        ],

        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_01",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: EDGE_CASE_NESTING_02_PARSED_TEXT, 
                spans: &[
                    TestSpan { start: 1, end: 6, style: SpanStyle::Italic, text: "a b c" },
                    TestSpan { start: 1, end: 4, style: SpanStyle::Italic, text: "a b" },
                    TestSpan { start: 3, end: 4, style: SpanStyle::Italic, text: "b" },
                ], language: None},
        ],
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_02",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: EDGE_CASE_NESTING_03_PARSED_TEXT, 
                spans: &[
                    TestSpan { start: 0, end: 5, style: SpanStyle::Bold, text: "a b c" }, 
                    TestSpan { start: 2, end: 3, style: SpanStyle::Italic, text: "b" },
                ], language: None},
        ],
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_03",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_04,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: EDGE_CASE_NESTING_04_PARSED_TEXT, 
                spans: &[
                    TestSpan { start: 0, end: 3, style: SpanStyle::Bold, text: "a b" }, 
                    TestSpan { start: 0, end: 1, style: SpanStyle::Italic, text: "a" },
                ], language: None},
        ],
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_04",
    },
    TestInputAndResult {
        source: EDGE_CASE_NESTING_05,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: EDGE_CASE_NESTING_05_PARSED_TEXT, 
                spans: &[
                    TestSpan { start: 0, end: 3, style: SpanStyle::Italic, text: "a b" }, 
                    TestSpan { start: 0, end: 1, style: SpanStyle::Bold, text: "a" },
                    TestSpan { start: 0, end: 1, style: SpanStyle::Italic, text: "a" },
                ], language: None},
        ],
        unique_tag: "EDGE_CASE_INPUT_TESTS::EDGE_CASE_NESTING_05",
    },
];

static HEADER_INVALID_TESTS: &'static [TestInvalidInputAndResult] = &[ 
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: HEADER_INVALID_01,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: HEADER_INVALID_01_PARSED_TEXT, 
                    spans: &[], language: None},
            ],
            unique_tag: "HEADER_INVALID_TESTS::HEADER_INVALID_01",
        }},
        error_messages: &["Line 1: invalid header level 7"],

    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: HEADER_INVALID_02,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: HEADER_INVALID_02_PARSED_TEXT, 
                    spans: &[
                        TestSpan { start: 9, end: 130, style: SpanStyle::Bold, 
                            text: "Thỏa Hiệp Án Fontainebleau 14/09/1946: ông Hồ cấu kết \
                                với Pháp để tiêu diệt các đảng quốc gia." }, 
                        TestSpan { start: 27, end: 51, style: SpanStyle::Italic, 
                            text: "Fontainebleau 14/09/1946" }, 
                        TestSpan { start: 93, end: 105, style: SpanStyle::Italic, 
                            text: "tiêu diệt" },
                    ], language: None},
            ],
            unique_tag: "HEADER_INVALID_TESTS::HEADER_INVALID_02",
        }},
        error_messages: &["Line 1: invalid header level 8"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: HEADER_INVALID_03,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: HEADER_INVALID_03, 
                    spans: &[], language: None},
            ],
            unique_tag: "HEADER_INVALID_TESTS::HEADER_INVALID_03",
        }},
        error_messages: &["Line 1: expected space after '#'"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: HEADER_INVALID_04,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: HEADER_INVALID_04_PARSED_TEXT, 
                    spans: &[
                        TestSpan { start: 3, end: 28, style: SpanStyle::Italic, 
                            text: "Giấc Mơ Trường Sơn" }
                    ], language: None},
            ],
            unique_tag: "HEADER_INVALID_TESTS::HEADER_INVALID_04",
        }},
        error_messages: &["Line 1: expected space after '#'"],
    },
];

static IMAGE_BLOCK_INVALID_TESTS: &'static [TestInvalidInputAndResult] = &[ 
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_INVALID_01,            
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: IMAGE_BLOCK_INVALID_01, 
                    spans: &[], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_INVALID_TESTS::IMAGE_BLOCK_INVALID_01",
        }},
        error_messages: &["Line 1: expected '['"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_INVALID_02,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: IMAGE_BLOCK_INVALID_02, 
                    spans: &[], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_INVALID_TESTS::IMAGE_BLOCK_INVALID_02",
        }},
        error_messages: &["Line 1: expected ']' after '['"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_INVALID_03,            
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: IMAGE_BLOCK_INVALID_03, 
                    spans: &[], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_INVALID_TESTS::IMAGE_BLOCK_INVALID_03",
        }},
        error_messages: &["Line 1: expected '('"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_INVALID_04,            
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: IMAGE_BLOCK_INVALID_04, 
                    spans: &[], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_INVALID_TESTS::IMAGE_BLOCK_INVALID_04",
        }},
        error_messages: &["Line 1: expected ')' after '('"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_INVALID_05,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: IMAGE_BLOCK_INVALID_05, 
                    spans: &[], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_INVALID_TESTS::IMAGE_BLOCK_INVALID_05",
        }},
        error_messages: &["Line 1: missing image path"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_INVALID_06,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: IMAGE_BLOCK_INVALID_06_PARSED_TEXT, 
                    spans: &[
                        TestSpan { start: 1, end: 60, style: SpanStyle::Bold, text: EMPHASIS_CAPTION_01_PARSED_TEXT },
                        TestSpan { start: 1, end: 11, style: SpanStyle::Italic, text: "Mount Fuji" },
                        TestSpan { start: 14, end: 43, style: SpanStyle::Italic, text: "富士山, ふじさ, Fujisan" },
                        TestSpan { start: 46, end: 60, style: SpanStyle::Italic, text: "Núi Phú Sỹ" },
                    ], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_INVALID_TESTS::IMAGE_BLOCK_INVALID_06",
        }},
        error_messages: &["Line 1: expected '['"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_INVALID_07,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: IMAGE_BLOCK_INVALID_07, 
                    spans: &[], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_INVALID_TESTS::IMAGE_BLOCK_INVALID_07",
        }},
        error_messages: &["Line 1: expected ')' after '('"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_INVALID_08,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: IMAGE_BLOCK_INVALID_08, 
                    spans: &[], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_INVALID_TESTS::IMAGE_BLOCK_INVALID_08",
        }},
        error_messages: &["Line 1: expected '['"],
    },
];

static IMAGE_BLOCK_SPACE_NESTED_INVALID_TESTS: &'static [TestInvalidInputAndResult] = &[ 
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_PARENS_IN_PATH_INVALID_01,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: IMAGE_BLOCK_PARENS_IN_PATH_INVALID_01, 
                    spans: &[], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_SPACE_NESTED_INVALID_TESTS::IMAGE_BLOCK_PARENS_IN_PATH_INVALID_01",
        }},
        error_messages: &["Line 1: expected ')' after '('"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_BRACKETS_IN_CAPTION_INVALID_01,
            expected_len: 1,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: IMAGE_BLOCK_BRACKETS_IN_CAPTION_INVALID_01, 
                    spans: &[], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_SPACE_NESTED_INVALID_TESTS::IMAGE_BLOCK_BRACKETS_IN_CAPTION_INVALID_01",
        }},
        error_messages: &["Line 1: expected ']' after '['"],
    },
    TestInvalidInputAndResult {
        result: { TestInputAndResult {
            source: IMAGE_BLOCK_BRACKETS_IN_CAPTION_INVALID_02,            
            expected_len: 2,
            expected_blocks: &[
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", text: "test\n", 
                    spans: &[], language: None},
                TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                    image_path: "", 
                    text: IMAGE_BLOCK_BRACKETS_IN_CAPTION_INVALID_02_PARSED_TEXT, 
                    spans: &[], language: None},
            ],
            unique_tag: "IMAGE_BLOCK_SPACE_NESTED_INVALID_TESTS::IMAGE_BLOCK_BRACKETS_IN_CAPTION_INVALID_02",
        }},
        error_messages: &["Line 2: expected ']' after '['"],
    },
];    

static THEMATIC_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: THEMATIC_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_01",
    },
    TestInputAndResult {
        source: THEMATIC_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_02",
    },
    TestInputAndResult {
        source: THEMATIC_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_03",
    },
    TestInputAndResult {
        source: THEMATIC_04,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_04",
    },
    TestInputAndResult {
        source: THEMATIC_05,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_05",
    },
    TestInputAndResult {
        source: THEMATIC_06,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_06",
    },
    TestInputAndResult {
        source: THEMATIC_07,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_07",
    },
    TestInputAndResult {
        source: THEMATIC_08,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_08",
    },
    TestInputAndResult {
        source: THEMATIC_09,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_09",
    },
    TestInputAndResult {
        source: THEMATIC_10,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_10",
    },
    TestInputAndResult {
        source: THEMATIC_11,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_11",
    },
    TestInputAndResult {
        source: THEMATIC_12,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_12",
    },
    TestInputAndResult {
        source: THEMATIC_13,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_14",
    },
    TestInputAndResult {
        source: THEMATIC_15,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_15",
    },
    TestInputAndResult {
        source: THEMATIC_16,
        expected_len: 3,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
            TestBlock { block_type: BlockType::Thematic, header_level: 0, 
                image_path: "", text: "", spans: &[], language: None},
        ],
        unique_tag: "THEMATIC_TESTS::THEMATIC_16",
    },
];

static FALSE_THEMATIC_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: FALSE_THEMATIC_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: FALSE_THEMATIC_01, spans: &[], language: None},
        ],
        unique_tag: "FALSE_THEMATIC_TESTS::FALSE_THEMATIC_01",
    },
    TestInputAndResult {
        source: FALSE_THEMATIC_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: FALSE_THEMATIC_02, spans: &[], language: None},
        ],
        unique_tag: "FALSE_THEMATIC_TESTS::FALSE_THEMATIC_02",
    },
    TestInputAndResult {
        source: FALSE_THEMATIC_03,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: FALSE_THEMATIC_03, spans: &[], language: None},
        ],
        unique_tag: "FALSE_THEMATIC_TESTS::FALSE_THEMATIC_03",
    },
    // TO_DO: FALSE_THEMATIC_04_PARSED_TEXT is not correct.
    TestInputAndResult {
        source: FALSE_THEMATIC_04,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: FALSE_THEMATIC_04_PARSED_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "FALSE_THEMATIC_TESTS::FALSE_THEMATIC_04",
    },
    TestInputAndResult {
        source: FALSE_THEMATIC_05,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: FALSE_THEMATIC_05_PARSED_TEXT, 
                spans: &[], language: None},
        ],
        unique_tag: "FALSE_THEMATIC_TESTS::FALSE_THEMATIC_05",
    },
];

static CODE_BLOCK_TESTS: &'static [TestInputAndResult] = &[ 
    TestInputAndResult {
        source: CODE_BLOCK_01,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Code, header_level: 0, 
                image_path: "", text: CODE_BLOCK_01_PARSED_CODE, 
                spans: &[], language: None},
        ],
        unique_tag: "CODE_BLOCK_TESTS::CODE_BLOCK_01",
    },
    TestInputAndResult {
        source: CODE_BLOCK_02,
        expected_len: 1,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Code, header_level: 0, 
                image_path: "", text: CODE_BLOCK_02_PARSED_CODE, 
                spans: &[], language: None},
        ],
        unique_tag: "CODE_BLOCK_TESTS::CODE_BLOCK_02",
    },
    TestInputAndResult {
        source: CODE_BLOCK_03,
        expected_len: 2,
        expected_blocks: &[
            TestBlock { block_type: BlockType::Code, header_level: 0, 
                image_path: "", text: CODE_BLOCK_03_PARSED_CODE, 
                spans: &[], language: None },
            TestBlock { block_type: BlockType::Paragraph, header_level: 0, 
                image_path: "", text: CODE_BLOCK_03_PARSED_TEXT, 
                spans: &[], language: None },
        ],
        unique_tag: "CODE_BLOCK_TESTS::CODE_BLOCK_03",
    },
];

static TEST_SET: &[&[TestInputAndResult]] = &[
    PLAIN_TEXT_TESTS,
    HEADER_TESTS,
    HEADER_EXAMPLE_70_VARIANT_TESTS,
    HASH_ESCAPE_TESTS,
    ASTERISK_TESTS,    
    NESTED_ASTERISK_TESTS,
    BUG_ASTERISK_TESTS,
    SPECIAL_ESCAPE_TESTS,
    EMPHASIS_UNEVEN_TOKEN_TESTS,
    IMAGE_BLOCK_TESTS,
    IMAGE_BLOCK_ESCAPE_TESTS,
    IMAGE_BLOCK_EMPHASIS_TESTS,
    IMAGE_BLOCK_EMPHASIS_ESCAPE_TESTS,
    IMAGE_BLOCK_MULTI_ITEMS_TESTS,
    IMAGE_BLOCK_ADJACENT_TEXT_TESTS,
    IMAGE_BLOCK_SPACE_NESTED_TESTS,
    HEADER_CAPTION_TEXT_TESTS,
    LEFT_RIGHT_FLANKING_TESTS,
    MODULO_3_TESTS,
    EDGE_CASE_INPUT_TESTS,
    THEMATIC_TESTS,
    FALSE_THEMATIC_TESTS,
    CODE_BLOCK_TESTS,
];

static TEST_INVALID_SET: &[&[TestInvalidInputAndResult]] = &[
    HEADER_INVALID_TESTS,
    IMAGE_BLOCK_INVALID_TESTS,
    IMAGE_BLOCK_SPACE_NESTED_INVALID_TESTS,
];

fn verify_inline_text(inline_text: &InlineContent, 
    test_block: &TestBlock, 
    unique_tag: &str, 
    type_name: &str,
    block_index: usize,
) {
    assert_eq!(inline_text.text(), test_block.text,
        "{} {} text {}", unique_tag, type_name, block_index+1);

    assert_eq!(inline_text.spans().len(), test_block.spans.len(), 
        "{} {} total number of text spans {}", unique_tag, 
        type_name, block_index+1);

    for (index, span) in inline_text.spans().iter().enumerate() {
        let expected_span = &test_block.spans[index];

        assert_eq!(span.start(), expected_span.start,
            "{} {} start {}", unique_tag, type_name, index+1);

        assert_eq!(span.end(), expected_span.end,
            "{} {} end {}", unique_tag, type_name, index+1);

        assert_eq!(span.style(), &expected_span.style,
            "{} {} style {}", unique_tag, type_name, index+1);

        assert_eq!(&inline_text.text()[span.start()..span.end()], expected_span.text,
            "{} {} span text {}", unique_tag, type_name, index+1);
    } 
}

fn verify_blocks(blocks: &[AstBlock], result: &TestInputAndResult) {
    assert_eq!(blocks.len(), result.expected_len, "{} Total number of blocks", result.unique_tag);

    for (index, block) in blocks.iter().enumerate() {
        let expected_block = &result.expected_blocks[index];

        match block {
            AstBlock::Header { level, content } => {
                assert_eq!(BlockType::Header, expected_block.block_type,
                    "{} block type {}", result.unique_tag, index+1);

                assert_eq!(*level, expected_block.header_level,
                    "{} header level {}", result.unique_tag, index+1);

                assert_eq!("", expected_block.image_path,
                    "{} image path {}", result.unique_tag, index+1);
                    
                verify_inline_text(&content, expected_block, 
                    result.unique_tag, "header", index);
            },

            AstBlock::Paragraph { content } => {
                assert_eq!(BlockType::Paragraph, expected_block.block_type,
                    "{} block type {}", result.unique_tag, index+1);

                assert_eq!(0, expected_block.header_level,
                    "{} header level {}", result.unique_tag, index+1);

                assert_eq!("", expected_block.image_path,
                    "{} image path {}", result.unique_tag, index+1);

                verify_inline_text(&content, expected_block, 
                    result.unique_tag, "paragraph", index);
            },

            AstBlock::Image { path, alt } => {
                assert_eq!(BlockType::Image, expected_block.block_type,
                    "{} block type {}", result.unique_tag, index+1);

                assert_eq!(*path, expected_block.image_path,
                    "{} image path {}", result.unique_tag, index+1);

                assert_eq!(0, expected_block.header_level,
                    "{} header level {}", result.unique_tag, index+1);

                verify_inline_text(&alt, expected_block, 
                    result.unique_tag, "image_block", index);
            },

            AstBlock::Thematic => {
                assert_eq!(BlockType::Thematic, expected_block.block_type,
                    "{} block type {}", result.unique_tag, index+1);
            },

            AstBlock::Code { language, content } => {
                assert_eq!(BlockType::Code, expected_block.block_type,
                    "{} block type {}", result.unique_tag, index+1);

                assert_eq!(*language, expected_block.language,
                    "{} language {}", result.unique_tag, index+1);

                assert_eq!(*content, expected_block.text,
                    "{} content {}", result.unique_tag, index+1);
            }
        };
    }
}

#[test]
fn test_parser_parse() {
    for group in TEST_SET {
        for test in *group {
            let mut scanner = Scanner::new(test.source);
            let res = scanner.scan_tokens();

            assert!(res.is_ok(), "Scanning {} should be valid", test.unique_tag);

            let tokens = res.unwrap();
            let mut parser = Parser::new(&tokens);
            let parse_output = parser.parse();

            assert!(!parse_output.has_error(), "1. Parsing {} should be valid", test.unique_tag);
            
            assert!(!parse_output.has_error(), 
                "2. Parsing {} should be valid", test.unique_tag);

            assert!(parse_output.errors().len() == 0, "Parsing {} 
                should not generate any error messages", test.unique_tag);

            verify_blocks(parse_output.blocks(), test);
        }
    }
}

#[test]
fn test_parser_parse_invalid() {
    for group in TEST_INVALID_SET {
        for test in *group {
            let mut scanner = Scanner::new(test.result.source);
            let res = scanner.scan_tokens();

            assert!(res.is_ok(), "Scanning {} should be valid", test.result.unique_tag);

            let tokens = res.unwrap();
            let mut parser = Parser::new(&tokens);
            let parse_output = parser.parse();

            assert!(parse_output.has_error(), 
                "Parsing {} invalid Markdowns should return errors", test.result.unique_tag);

            assert!(parse_output.errors().len() > 0, "Parsing {} 
                should generate some error messages", test.result.unique_tag);

            assert!(parse_output.errors().len() > 0, 
                "Parsing {} invalid Markdowns should set error messages", test.result.unique_tag);

            for (index, s) in parse_output.errors().iter().enumerate() {
                assert_eq!(s, test.error_messages[index], 
                    "{} error {} message", test.result.unique_tag, index);
            }

            verify_blocks(parse_output.blocks(), &test.result);
        }
    }
}