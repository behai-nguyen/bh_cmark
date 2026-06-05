/* 15/05/2025 */

//! Generating HTML from [`crate::parser::base::ParseOutput`].

use std::cmp::Ordering;

use crate::ast::{Span, SpanStyle};

#[derive(Debug, PartialEq, Eq)]
enum Edge {
    Start,
    End,
}

#[derive(Debug)]
struct Event {
    index: usize,
    edge: Edge,
    style: SpanStyle,
    span_len: usize,
}

fn style_priority_start(style: &SpanStyle) -> usize {
    match style {
        SpanStyle::Italic => 0,
        SpanStyle::Bold => 1,
        _ => unimplemented!(),
    }    
}

fn style_priority_end(style: &SpanStyle) -> usize {
    match style {
        SpanStyle::Bold => 0,
        SpanStyle::Italic => 1,
        _ => unimplemented!(),
    }
}

/// # Algorithm
/// 
///   1. Transform each Span into two separate events: a Start event and an End event.
/// 
///   2. Sort all events primarily by their byte index (ascending). If the indexes are 
///      equal, apply strict tie-breaking rules:
/// 
///      * An End event must always come before a Start event to close inner tags 
///        before opening new ones.
/// 
///      * For two Start events at the same index, the one with the larger span (outermost) 
///        must come first. 
/// 
///      * For two End events at the same index, the one with the smaller span (innermost) 
///        must come first.
pub fn spans_to_html(clean_text: &str, spans: &[Span]) -> String {
    let mut events = Vec::new();

    // 1. Flatten spans into start and end events.
    for span in spans {
        let len = span.end() - span.start();
        events.push(Event {
            index: span.start(),
            edge: Edge::Start,
            style: span.style().clone(),
            span_len: len,
        });
        events.push(Event {
            index: span.end(),
            edge: Edge::End,
            style: span.style().clone(),
            span_len: len,
        });
    }

    // 2. Sort events with strict tie-breaking rules.
    events.sort_by(|a, b| {
        if a.index != b.index {
            return a.index.cmp(&b.index);
        }
        
        match (&a.edge, &b.edge) {
            // End tags come before Start tags at the same position.
            (Edge::End, Edge::Start) => Ordering::Less,
            (Edge::Start, Edge::End) => Ordering::Greater,
            
            // Two Start tags: Outermost span (largest len) goes first.
            (Edge::Start, Edge::Start) => {
                b.span_len.cmp(&a.span_len)
                    .then_with(|| style_priority_start(&a.style).cmp(&style_priority_start(&b.style)))
            }            
            
            // Two End tags: Innermost span (smallest len) goes first.
            (Edge::End, Edge::End) => {
                a.span_len.cmp(&b.span_len)
                    .then_with(|| style_priority_end(&a.style).cmp(&style_priority_end(&b.style)))
            }
        }
    });

    // 3. Reconstruct the string linearly from left to right.
    let mut html = String::new();
    let mut current_byte = 0;

    for event in events {
        // Append text slice up to the current event index.
        if event.index > current_byte {
            html.push_str(&clean_text[current_byte..event.index]);
            current_byte = event.index;
        }

        // Append the appropriate HTML tag.
        match (event.edge, event.style) {
            (Edge::Start, SpanStyle::Bold) => html.push_str("<strong>"),
            (Edge::Start, SpanStyle::Italic) => html.push_str("<em>"),
            (Edge::End, SpanStyle::Bold) => html.push_str("</strong>"),
            (Edge::End, SpanStyle::Italic) => html.push_str("</em>"),
            (_, _) => {}
        }
    }

    // Append any remaining text after the last tag.
    if current_byte < clean_text.len() {
        html.push_str(&clean_text[current_byte..]);
    }

    html
}

pub fn header_to_html(level: u8, clean_text: &str, spans: &[Span]) -> String {
    let html = spans_to_html(clean_text, spans);

    if html.ends_with('\n') {
        format!("<h{0}>{1}</h{0}>\n", level, html.trim_end_matches('\n').trim())
    } else {
        format!("<h{0}>{1}</h{0}>", level, html.trim())
    }
}

pub fn thematic_to_html() -> String {
    "<hr />".to_string()
}

pub fn code_to_html(_language: &Option<String>, content: &str) -> String {
    format!("<pre><code>{}</code></pre>", content)
}