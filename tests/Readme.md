<!-- 09/04/2026. -->

**bh_cmark** — Be Hai's CommonMark implementation — test suite.

# Testing Order

This is also the order the code were written.

## Module test_scanner.rs

#### cargo test --test test_scanner [-- --nocapture]
#### cargo test test_scanner -- --exact [--nocapture]

## Module test_delimiter_pre_process.rs

#### cargo test --test test_delimiter_pre_process [-- --nocapture]
#### cargo test test_delimiter_parser_pre_process -- --exact [--nocapture]

## Module test_delimiter_classify_can_open_close.rs

#### cargo test --test test_delimiter_classify_can_open_close [-- --nocapture]
#### cargo test test_delimiter_parser_classify_can_open_close -- --exact [--nocapture]

## Module test_delimiter_match_delimiters.rs

#### cargo test --test test_delimiter_match_delimiters [-- --nocapture]
#### cargo test test_delimiter_parser_match_delimiters -- --exact [--nocapture]

## Module test_delimiter_map_markdown_to_clean.rs

#### cargo test --test test_delimiter_map_markdown_to_clean [-- --nocapture]
#### cargo test test_delimiter_map_markdown_to_clean -- --exact [--nocapture]

## Module test_delimiter_produce_spans.rs

#### cargo test --test test_delimiter_produce_spans [-- --nocapture]
#### cargo test test_delimiter_parser_produce_spans -- --exact [--nocapture]

## Module test_delimiter_parse_inline.rs

#### cargo test --test test_delimiter_parse_inline [-- --nocapture]
#### cargo test test_delimiter_parser_parse_inline -- --exact [--nocapture]

## Module test_parser.rs

#### cargo test --test test_parser [-- --nocapture]

#### cargo test test_parser_parse -- --exact [--nocapture]
#### cargo test test_parser_parse_invalid -- --exact [--nocapture]

## Module test_commonmark_spec.rs

#### cargo test --test test_commonmark_spec [-- --nocapture]
#### cargo test test_spec_json -- --exact [--nocapture]