/* 17/05/2026 */

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JSONSpec {
    markdown: String,
    html: String,
    example: usize,
    start_line: usize,
    end_line: usize,
    section: String,
}

impl JSONSpec {
    pub fn markdown(&self) -> &str {
        &self.markdown
    }

    pub fn html(&self) -> &str {
        &self.html
    }

    pub fn example(&self) -> usize {
        self.example
    }

    pub fn start_line(&self) -> usize {
        self.start_line
    }

    pub fn end_line(&self) -> usize {
        self.end_line
    }

    pub fn section(&self) -> &str {
        &self.section
    }
}