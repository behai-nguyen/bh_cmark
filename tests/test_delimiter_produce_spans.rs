/* 23/04/2026 */

//! Test [`bh_cmark::parser::delimiter::DelimiterRun`]'s `produce_spans()` method.
//! 
//! Test only inline Markdown texts, not block elements.
//! 
//! To run test for this module only:
//!
//!     * cargo test --test test_delimiter_produce_spans
//!
//! To run a specific test method:
//!
//!     * cargo test test_delimiter_parser_produce_spans -- --exact [--nocapture]
//!

use std::collections::HashSet;

use bh_cmark::scanner::Scanner;
use bh_cmark::parser::delimiter::{
    DelimiterParser,
};

use bh_cmark::ast::Span;

mod common;
use common::test_data_produce_spans_parse_inline::{
    TestInputAndResult,
    TEST_SET
};

fn verify_runs(clean_text: &str,
    spans: &Vec<Span>,
    result: &TestInputAndResult
) {
    assert_eq!(clean_text, result.clean_text, "{} clean_text", result.unique_tag);
    assert_eq!(spans.len(), result.expected_len, "{} Total number of spans", result.unique_tag);

    for (index, span) in spans.iter().enumerate() {
        let expected_span = &result.expected_spans[index];

        assert_eq!(span.start(), expected_span.start,
            "{} start {}", result.unique_tag, index+1);

        assert_eq!(span.end(), expected_span.end,
            "{} end {}", result.unique_tag, index+1);

        assert_eq!(span.style(), &expected_span.style,
            "{} style {}", result.unique_tag, index+1);

        assert_eq!(&clean_text[span.start()..span.end()], expected_span.text,
            "{} span text {}", result.unique_tag, index+1);
    }
}

#[test]
fn test_delimiter_parser_produce_spans() {
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

            let res = parser.map_markdown_to_clean();
            assert!(res.is_ok(), "map_markdown_to_clean() should be valid");
            let clean_text = res.unwrap();
            
            let res = parser.produce_spans();
            assert!(res.is_ok(), "produce_spans() should be valid");

            verify_runs(&clean_text, &res.unwrap(), test);
        }
    }
}