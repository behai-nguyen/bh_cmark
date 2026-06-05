/* 17/05/2026. */

//! Selects and tests parsing a handful of examples from 
//! https://spec.commonmark.org/0.31.2/spec.json.
//! 
//! # Note 
//! 
//!   The test results are logged to `test_0.31.2-spec.md`.
//! 
//! To run test for this module only:
//!
//!     * cargo test --test test_commonmark_spec
//!
//! To run a specific test method:
//!
//!     * cargo test test_spec_json -- --exact [--nocapture]
//!

use std::{fs, fs::File, process};
use std::io::prelude::*; // Imports the Write trait

use time::{OffsetDateTime, format_description};

use bh_cmark::ast::AstBlock;
use bh_cmark::scanner::Scanner;
use bh_cmark::parser::base::ParseOutput;
use bh_cmark::parser::parser::Parser;
use bh_cmark::render::html::{
    spans_to_html,
    header_to_html,
    thematic_to_html,
    code_to_html,
};

mod common;
use common::json_spec::JSONSpec;

#[derive(Debug)]
struct SpecJSONFilter<'a> {
    section: &'a str,
    excluded_tokens: &'a [&'a str],
}

/// Note, the parser can report errors, but it will attempt 
/// to process the syntax-error Markdown as normal paragraph, 
/// and hence the results/HTMLs might match that of the specs.
static SPECS_TEST_SET: &'static [SpecJSONFilter<'static>] = &[ 
    // 58 tests: all pass.
    SpecJSONFilter {
        section: "Emphasis and strong emphasis",
        excluded_tokens: &["_", "[", "]", "http", "`", "img", "href"],
    },
    // 22 tests: 21 failed. They are inline images which are not 
    //     supported yet.
    SpecJSONFilter {
        section: "Images",
        excluded_tokens: &[],
    },
    // 18 tests: all pass.
    SpecJSONFilter {
        section: "ATX headings",
        excluded_tokens: &[],
    },
];

fn log_test_datetime(file: &mut File) {
    let now = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc());
    let format = format_description::parse("[day]/[month]/[year] [hour]:[minute]:[second]").unwrap();

    file.write(format!("# Test Date and Time: {}\n\n", 
        now.format(&format).unwrap())
        .as_bytes()).expect("Failed to log test datetime");
    file.flush().expect("Failed to flush test datetime");
}

fn log_test_header(spec_json_filter: &SpecJSONFilter, total_tests: usize, file: &mut File) {
    file.write(format!("# Section \"{}\" -- {} Tests\n", 
        spec_json_filter.section, total_tests)
        .as_bytes()).expect("Failed to log test header");
    file.write(format!("## Excluded Tokens: {}\n", 
        spec_json_filter.excluded_tokens.join(", "))
        .as_bytes()).expect("Failed to log test header");

    file.flush().expect("Failed to flush test header");
}

fn log_json_spec(json_spec: &JSONSpec, file: &mut File) {
    file.write(format!("Example: {}, Markdown: \"{}\"\n", json_spec.example(), 
            json_spec.markdown().replace("\n", "\\n"))
        .as_bytes()).expect("Failed to log JSON spec test");

    file.flush().expect("Failed to flush JSON spec test");
}

fn log_parser_result(has_error: bool, parser_output: &ParseOutput, file: &mut File) {
    (if has_error {
        file.write(format!("    ❌ Parser errors: [{}]\n", 
            parser_output.errors().join(" / ")).as_bytes())
    } else {
        file.write(format!("    {} Parsing was successful\n", "✔️").as_bytes())
    }).expect("Failed to log parser result");

    file.flush().expect("Failed to flush parser result");
}

fn log_test_result(json_spec: &JSONSpec, 
    final_html: &str, 
    htmls_matched: bool, 
    file: &mut File
) {
    let matched_symbol = if htmls_matched { "✔️" } else { "❌" };

    file.write(format!("    Spec.  HTML: \"{}\"\n", json_spec.html().replace("\n", "\\n"))
        .as_bytes()).expect("Failed to write test result");
    file.write(format!("    Actual HTML: \"{}\"\n", final_html.replace("\n", "\\n"))
        .as_bytes()).expect("Failed to write test result");
    
    file.write(format!("    {} Actual HTML == Spec.  HTML: {}\n", 
        matched_symbol, htmls_matched)
        .as_bytes()).expect("Failed to write test result");

    file.flush().expect("Failed to flush test result");
}

fn log_test_footer(test_count: usize, 
    parser_error_count: usize, 
    result_failed_count: usize, 
    file: &mut File
) {
    file.write(format!("Test count: {}\nParser error count: {}\nFailed result count: {}\n", 
        test_count, parser_error_count, result_failed_count)
        .as_bytes()).expect("Failed to write test footer");
    file.flush().expect("Failed to flush  test footer");
}

/// Format the rendered HTML to look like that of the specs.:
/// https://spec.commonmark.org/0.31.2/spec.json.
fn post_process_html(html: &str, p_wrapped: bool) -> String {
    if html.is_empty() {
        return String::new();
    }

    let (open_tag, close_tag) = 
        if p_wrapped { ("<p>", "</p>") } else { ("", "") };

    // Always blindly appends `\n`!
    let result = 
        format!("{}{}{}\n", open_tag, html.trim_end_matches('\n'), close_tag);

    result.replace("\"", "&quot;")
}

fn do_test_spec_json(spec_json_filter: &SpecJSONFilter, 
    specs_json: &Vec<JSONSpec>, 
    file: &mut File
) {
    let test_specs_json: Vec<&JSONSpec> = specs_json.iter()
        .filter(|u| u.section() == spec_json_filter.section)
        .filter(|u| 
            !spec_json_filter.excluded_tokens.iter()
            .any(|&token| u.markdown().contains(token)))
        .collect();

    log_test_datetime(file);    
    log_test_header(spec_json_filter, test_specs_json.len(), file);

    let mut parser_error_count: usize = 0;
    let mut result_failed_count: usize = 0;

    let collect_html = |html: &str, final_html: &mut String| {
        if final_html.len() > 0 && !final_html.ends_with('\n') {
            final_html.push('\n');
        }

        final_html.push_str(&html);
    };

    // The actual tests.
    for tsj in &test_specs_json {
        log_json_spec(tsj, file);

        let mut scanner = Scanner::new(tsj.markdown());
        let res = scanner.scan_tokens();

        assert!(res.is_ok(), "Scanning should be valid");

        let tokens = res.unwrap();
        let mut parser = Parser::new(&tokens);
        let parse_output = parser.parse();

        log_parser_result(parse_output.has_error(), &parse_output, file);

        if parse_output.has_error() { parser_error_count += 1; }

        let mut final_html = String::new();

        for block in parse_output.blocks() {
            match block {
                AstBlock::Header { level, content } => {
                    let mut html = header_to_html(*level, &content.text(), &content.spans());
                    html = post_process_html(&html, false);

                    collect_html(&html, &mut final_html);
                }
                AstBlock::Paragraph { content } => {
                    // True HTML for the input Markdown.
                    let mut html = spans_to_html(&content.text(), &content.spans());
                    html = post_process_html(&html, true);

                    collect_html(&html, &mut final_html);
                }
                AstBlock::Image { path: _, alt: _ } => {}
                AstBlock::Thematic => {
                    let mut html = thematic_to_html();
                    html = post_process_html(&html, false);

                    collect_html(&html, &mut final_html);
                }
                AstBlock::Code { language, content } => {
                    let mut html = code_to_html(&language, &content);
                    html = post_process_html(&html, false);

                    collect_html(&html, &mut final_html);
                }                
            }
        }

        // Do not call assert_eq!(final_html, tsj.html(), "Example {} html", tsj.example());
        // let all tests take place, just log the results.

        let htmls_matched = final_html == tsj.html();
        
        if !htmls_matched { result_failed_count += 1; }

        log_test_result(&tsj, &final_html, htmls_matched, file);
    }

    log_test_footer(test_specs_json.len(), parser_error_count, result_failed_count, file);
}

#[test]
fn test_spec_json() {
    // Read input text file.
    let json = match fs::read_to_string("./tests/data/0.31.2-spec.json") {
        Ok(str) => str,
        Err(err) => { 
            println!("{}", err.to_string());
            process::exit(1);
        }
    };

    let mut file = File::create("test_0.31.2-spec.md").expect("Failed to create output text file");

    // The type annotation <Vec<JSONSpec>> tells Serde how to parse the data
    let specs_json: Vec<JSONSpec> = serde_json::from_str(&json).expect("Failed to parse JSON");

    for json_spec_filter in SPECS_TEST_SET {
        do_test_spec_json(json_spec_filter, &specs_json, &mut file);
    }
}