/* 28/05/2026 */

//! # Side Effects
//!
//! Advances `current` while inspecting the current line.
//!
//! On detecting:
//!
//! * [`DetectedBlock::Thematic`] — leaves `current` on the
//!   terminating newline or EOF.
//!
//! * [`DetectedBlock::Code`] — leaves `current` immediately
//!   after the fourth leading space.
//!
//! * [`DetectedBlock::None`] — restores `current` to its
//!   original position.

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

use crate::token_type::TokenType;

/// Thematic: thematic break `<hr />`.
/// Code: `<code_block>...</code_block>` defined, not implemented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockDetector {
    None,
    ThematicBreak,
    IndentedCode,
}

#[derive(Debug, Eq)]
pub struct MarkerRun {
    token_type: TokenType,
    count: usize,
}

// Only hash and compare the 'id' field
impl PartialEq for MarkerRun {
    fn eq(&self, other: &Self) -> bool { self.token_type == other.token_type }
}

impl Hash for MarkerRun {
    fn hash<H: Hasher>(&self, state: &mut H) { self.token_type.hash(state); }
}

/// This allows .get(&TokenType)
impl Borrow<TokenType> for MarkerRun {
    fn borrow(&self) -> &TokenType { &self.token_type }
}

impl MarkerRun {
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

/// Supports:
///     `** ** ** ** **`
///     `* * *`
///     `* ** *******   *******`
///     `    ****`
///     ` __ __ __ `
///     `   --- --- ---`
pub struct MarkerRunAnalyser {
    /// Entries in this vector are only of two [`TokenType`]s: 
    /// [`TokenType::Whitespace`] and either [`TokenType::Star`],
    /// [`TokenType::Underscore`], or [`TokenType::Dash`].
    /// 
    /// That means, it contains at most two entries: one is 
    /// [`TokenType::Whitespace`], the other one is one of the three
    /// mentioned above.
    marker_runs: HashSet<MarkerRun>,
}

impl MarkerRunAnalyser {
    pub fn new() -> Self {
        MarkerRunAnalyser {
            marker_runs: HashSet::new(),
        }
    }

    /// Register a [`TokenType`]. A registration include only 
    /// [`TokenType::Whitespace`] and either [`TokenType::Star`],
    /// [`TokenType::Underscore`], or [`TokenType::Dash`] only.
    /// 
    /// "Register" means: if the incoming `token_type` is not in 
    /// the `self.marker_runs`, then create an entry with the 
    /// `MarkerRun::count` of 1, if `token_type` already exists 
    /// increased `MarkerRun::count` by 1.
    /// 
    /// Return the updated `MarkerRun::count`.
    /// 
    /// # Example
    /// 
    ///     `** ** ** ** **` results in two entries [`TokenType::Whitespace`], 
    ///     count of 4, and [`TokenType::Star`] count of 10.
    pub fn register(&mut self, token_type: TokenType) -> usize {
        if let Some(mut marker_run) = self.marker_runs.take(&token_type) {
            marker_run.count += 1; 
            let result = marker_run.count;
            self.marker_runs.insert(marker_run);
            result
        } else {
            self.marker_runs.insert(MarkerRun 
                { token_type: token_type, count: 1 });
            1
        }
    }

    pub fn only_one_run(&self) -> bool {
        self.marker_runs.len() == 1
    }

    /// None whitespace means, either `*`, `_`, or `-` and nothing else.
    pub fn non_whitespace_count(&self) -> usize {
        for marker_run in &self.marker_runs {
            if marker_run.token_type != TokenType::Whitespace {
                return marker_run.count;
            }
        }

        0
    }
}