/* 25/01/2026 */

//! Supporting Markdown AST.

use std::fmt;

/// Markdown supports six header levels.
pub const MAX_HEADER_LEVEL: usize = 6;

/// Overlapping bold and italic spans imply bold-italic rendering.
/// Intention: Bold > Italic.
#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone)]
pub enum SpanStyle {
    Normal,
    Bold,
    Italic,
}

/// Individual slices within the paragraph text with different style.
/// `Span` can be adjacent or overlapped / nested.
/// 
/// The below markdown results in adjacent `Span`s:
///      "— **Tưởng Vĩnh Kính**, Hồ Chí Minh Tại *Trung Quốc*, Thượng Huyền dịch, \
///       ***trang 339***."
/// 
/// Which would produces the `Span`s:
///     [
///         Span { start: 0, end: 4, style: Normal }
///         Span { start: 4, end: 24, style: Bold }
///         Span { start: 24, end: 47, style: Normal }
///         Span { start: 47, end: 59, style: Italic }
///         Span { start: 59, end: 87, style: Normal }
///         Span { start: 87, end: 96, style: Bold }
///         Span { start: 87, end: 96, style: Italic }
///         Span { start: 96, end: 97, style: Normal }
///     ]
/// 
/// The following markdown results in overlapped / nested `Span`s:
///     "**Không đọc *sử* không đủ tư cách nói chuyện *chính trị*.**"
/// 
/// And it produces the `Span`s:
///     [
///         Span { start: 0, end: 69, style: Bold }, 
///         Span { start: 14, end: 18, style: Italic }, 
///         Span { start: 56, end: 68, style: Italic }
///     ]
#[derive(Debug, Clone)]
pub struct Span {
    /// The start byte of a text slice with a a specific style.
    start: usize,
    /// The end byte of a text slice with a a specific style.
    end: usize,
    /// The style of the text slice indexed by `start`..`slice`.
    style: SpanStyle,
}

impl Span {
    pub fn new(start: usize, 
        end: usize,
        marker_count: u8,
    ) -> Self {
        let style= match marker_count {
            1 => SpanStyle::Italic,
            2 => SpanStyle::Bold,
            _ => SpanStyle::Normal,
        };

        Span { start, end, style, }
    }

    pub fn normal(start: usize, end: usize) -> Self {
        Span { start, end, style: SpanStyle::Normal }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn style(&self) -> &SpanStyle {
        &self.style
    }
}

/// Represents the inline [`md_parser::parse::delimeter::DelimiterParser`]'s 
/// final result.
#[derive(Debug, Clone)]
pub struct InlineContent {
    /// Final text without actual consumed delimiters.
    text: String,
    /// Emphasis [`Span`], i.e. bold, italic byte ranges if applicable 
    /// for `text`. This vector could be empty, and it is still a valid
    /// result.
    spans: Vec<Span>,
}

impl InlineContent {
    pub fn new(text: String, spans: Vec<Span>) -> Self {
        Self { text, spans }
    }

    pub fn new_empty() -> Self {
        Self { text: "".to_string(),
            spans: Vec::<Span>::new() }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn spans(&self) -> &Vec<Span> {
        &self.spans
    }
}

impl fmt::Display for InlineContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "text: {}", self.text)?;        
        for span in &self.spans {
            writeln!(f, "span: {:?} ➜ {}", span, &self.text[span.start..span.end])?;
        }

        Ok(())
  }
}

/// Semantic document structure.
#[derive(Debug, Clone)]
pub enum AstBlock {
    /// The text block / line is a header.
    Header { level: u8, content: InlineContent },
    /// `text`: the clean text block / paragraph / blank line is a normal text.
    /// `spans`: byte-ranges and their styles for slices in `text`.
    Paragraph { content: InlineContent },
    /// Encapsulates `![caption](relative/path/to/image.png)`.
    /// `path`: `relative/path/to/image.png`.
    /// 'caption`: `caption`.
    Image { path: String, alt: InlineContent },
    /// Represents a thematic break / horizontal rule (e.g., `***`, `---`, `___`).
    Thematic, 
    /// Holds optional language identifier and the raw code string.
    /// Note: not implemented yet.
    Code { language: Option<String>, content: String },
}

impl AstBlock {
    pub fn header(level: u8, content: InlineContent) -> Self {
        AstBlock::Header { level, content }
    }

    pub fn paragraph(content: InlineContent) -> Self {
        AstBlock::Paragraph { content }
    }

    pub fn image(path: String, 
        alt: InlineContent,
    ) -> Self {
        AstBlock::Image { path, alt }
    }

    pub fn thematic() -> Self {
        AstBlock::Thematic {}
    }

    pub fn code(language: &str, 
        content: String,
    ) -> Self {
        AstBlock::Code { 
            language: if language.len() > 0 { Some(language.to_string()) }
                else { None },
            content 
        }
    }
}
