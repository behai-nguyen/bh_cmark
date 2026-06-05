/* 25/04/2026 */

//! Test [`bh_cmark::parser::delimiter::DelimiterRun`]'s `parse_inline()` method.
//! 
//! Test only inline Markdown texts, not block elements.
//! 
//! To run test for this module only:
//!
//!     * cargo test --test test_delimiter_parse_inline
//!
//! To run a specific test method:
//!
//!     * cargo test test_delimiter_parser_parse_inline -- --exact [--nocapture]
//!

use std::collections::HashSet;

use bh_cmark::scanner::Scanner;
use bh_cmark::ast::InlineContent;
use bh_cmark::parser::delimiter::DelimiterParser;

mod common;
use common::test_data_produce_spans_parse_inline::{
    TestInputAndResult,
    TEST_SET
};

fn verify_runs(inline_text: &InlineContent,
    result: &TestInputAndResult
) {
    assert_eq!(inline_text.text(), result.clean_text, "{} clean_text", result.unique_tag);
    assert_eq!(inline_text.spans().len(), result.expected_len, "{} Total number of spans", result.unique_tag);

    for (index, span) in inline_text.spans().iter().enumerate() {
        let expected_span = &result.expected_spans[index];

        assert_eq!(span.start(), expected_span.start,
            "{} start {}", result.unique_tag, index+1);

        assert_eq!(span.end(), expected_span.end,
            "{} end {}", result.unique_tag, index+1);

        assert_eq!(span.style(), &expected_span.style,
            "{} style {}", result.unique_tag, index+1);

        assert_eq!(&inline_text.text()[span.start()..span.end()], expected_span.text,
            "{} span text {}", result.unique_tag, index+1);
    }
}

#[test]
fn test_delimiter_parser_parse_inline() {
    for group in TEST_SET {
        for test in *group {
            let mut scanner = Scanner::new(test.source);
            let res = scanner.scan_tokens();

            assert!(res.is_ok(), "Scanning {} should be valid", test.unique_tag);

            let tokens = res.unwrap();
            let ignored_tokens: HashSet<usize> = HashSet::new();
            let mut parser = DelimiterParser::new(&tokens, &ignored_tokens);

            let res = parser.parse_inline();
            assert!(res.is_ok(), "parse_inline() {} should be valid", test.unique_tag);

            verify_runs(&res.unwrap(), test);
        }
    }
}