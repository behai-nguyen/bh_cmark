/* 08/04/2026. */

//! Process inline emphasis. The API is `parse_inline()`.
//! 
//! # Remaining Issues
//! 
//! 1. The remaining differences from full CommonMark are now:
//! 
//!    * `_` underscore restrictions.
//!    * opener_bottom optimisation.
//! 
//! 2. To be completed:
//! 
//!    * edge-case conformance.
//!    * inactive delimiter semantics: i.e. preventing `[foo [bar](url1)](url2)` 
//!      from forming nested links.
//!    * exhaustive differential testing: compare an exhaustive test result set 
//!      against a preference Markdown parser such as `cmark` or `commonmark.js`.
//! 
//! # Warning
//! 
//! * The current `calculate_and_set_text_index()` implementation assumes that
//!   a delimiter run participates only as an opener or only as a closer.
//!   If a delimiter run is matched in both roles, `text_index` calculation will
//!   be incorrect and the algorithm must be extended to account for both roles.
//! 
//! * Dual-role delimiter runs are not yet supported.

use std::collections::HashSet;

use crate::token_type::TokenType;
use crate::parser::base::{
    END_OF_INPUT_CHAR, 
    ParseResult,
    BaseParser,
};

use crate::token::Token;
use crate::ast::{InlineContent, Span};

/// A `run` is defined as a group of emphases, such as `*`, `**`, `___`, etc.
/// 
/// For example, in `***a *b* c***`, there are four `run`s: `***`, `*`, `*` 
/// and `***`. A `run` can be an open run (an opener), or a close run 
/// (a closer), a `run` can also be both an opener and a closer, or it can 
/// also be neither. Markdown `***a *b* c***` results in the following **after** 
/// [`DelimiterParser`]'s `classify_can_open_close()`, but **before** 
/// `match_delimiters()`:
/// 
/// ```rust
/// { token_type: Star, token_index: 0, byte_index: 3, count: 3, can_open: true, can_close: false, remaining: 3 }
/// { token_type: Star, token_index: 4, byte_index: 6, count: 1, can_open: true, can_close: false, remaining: 1 }
/// { token_type: Star, token_index: 6, byte_index: 8, count: 1, can_open: false, can_close: true, remaining: 1 }
/// { token_type: Star, token_index: 8, byte_index: 13, count: 3, can_open: false, can_close: true, remaining: 3 }
/// ```
/// `match_delimiters()` updates `remaining` after consuming emphases during 
/// matching. In this case, `remaining` will be reducing to 0. If a `remaining`
/// is not 0, that means the `remaining` [`Token`] `lexeme`s will become literals, 
/// and output as part of the final clean text -- clean means without emphases.
#[derive(Default, Debug, Clone)]
pub struct DelimiterRun {
    token_type: TokenType,	
    /// [`Token`] vector index of the first emphasis of a run.
    token_index: usize,    
	/// The byte index to the underlying [`Token`] `lexeme`. 
    /// Note: it is the byte position of the last emphasis in the run.
    byte_index: usize,    
	/// The original count of consecutive emphases. This value is set once 
    /// at creation and stays constant throughout.
    count: u8,	
    /// A possible open run: an opener.
    can_open: bool,
    /// A possible close run: a closer.
    can_close: bool,
    /// The number of left over emphases after opener-closer match. Its 
    /// value is set to `count`'s value, and gets updated during matching.
    remaining: u8,
    /// 🦀 `ignored_byte_count`: Some `Token`s can be ignored: not to be 
    /// included in the final clean text and `Span`s generation. If they 
    /// are in front, i.e., their indexes are less than `token_index` value, 
    /// `ignored_byte_count` is the total byte count of all of those ignored
    /// `Token`s.
    /// 
    /// The actual byte index to the text. It is only set at the span
    /// generation stage, where delimiter runs matching has been completed.
    /// It is calculated as:
    /// 
    ///     opener: byte_index - Σ (count - remaining) - Σ ignored_byte_count
    ///     closer: byte_index - Σ count - Σ ignored_byte_count
    /// 
    /// Why is the difference? Remaining delimiters are literals, and they are 
    /// not part of the emphasis, that is they fall outside `<strong>..</strong>`
    /// and `<em>..</em>`. For opener runs, they appear **in front of** the 
    /// delimited text, so their byte occurences must be accounted for in the 
    /// final clean text. While for closer runs, they appear **after** the 
    /// delimited text, their byte occurences are not applicable to their own
    /// delimited text: but must be accounted for, for the next group since we 
    /// are going from left to right.
    text_index: usize,
}

pub type DelimiterRunVector = Vec<DelimiterRun>;

/// List of matching [`DelimiterRun`] opener and closer, and how many 
/// emphases are there: 2 or 1, that is bold or italic.
/// 
/// The Markdown `***a *b* c***` listed in [`DelimiterRun`] above, would 
/// result in the following `DelimiterMatch`:
/// 
/// ```rust
/// DelimiterMatch { opener_index: 0, closer_index: 3, use_count: 2 }
/// DelimiterMatch { opener_index: 0, closer_index: 3, use_count: 1 }
/// DelimiterMatch { opener_index: 1, closer_index: 2, use_count: 1 }
/// ```
/// 
/// The first opener `***` matches the last closer `***` as **strong** 
/// and *italic*. 
/// 
/// The second opener `*` matches the third closer `*` as *italic*.
/// 
/// Final rendering  `<em><strong>a <em>b</em> c</strong></em>`.
#[derive(Debug, Eq, PartialEq)]
pub struct DelimiterMatch {
    /// Index to an opening [`DelimiterRun`].
    opener_index: usize,
    /// Index to a closing [`DelimiterRun`].
    closer_index: usize,
    /// The matched number of emphases: 2 for **bold**, 1 for *italic*.
    use_count: u8,
}

pub type DelimiterMatchVector = Vec<DelimiterMatch>;

pub struct DelimiterParser<'a> {
    base: BaseParser<'a>,
    /// Indexes of `Token`s not to be included in the final 
    /// clean text and `Span`s.
    ignored_tokens: &'a HashSet<usize>,
    run_vector: DelimiterRunVector,
    match_vector: DelimiterMatchVector,
}

impl DelimiterRun {
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn token_index(&self) -> usize {
        self.token_index
    }

    pub fn byte_index(&self) -> usize {
        self.byte_index
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn can_open(&self) -> bool {
        self.can_open
    }

    pub fn can_close(&self) -> bool {
        self.can_close
    }

    pub fn remaining(&self) -> u8 {
        self.remaining
    }

    pub fn text_index(&self) -> usize {
        self.text_index
    }

    pub fn set_token_index(&mut self, val: usize) {
        self.token_index = val;
    }

    pub fn set_byte_index(&mut self, val: usize) {
        self.byte_index = val;
    }
}

impl DelimiterMatch {
    pub fn new(opener_index: usize, 
        closer_index: usize, 
        use_count: u8
    ) -> Self {
        Self { opener_index, closer_index, use_count }
    }

    pub fn opener_index(&self) -> usize {
        self.opener_index
    }

    pub fn closer_index(&self) -> usize {
        self.closer_index
    }

    pub fn use_count(&self) -> u8 {
        self.use_count
    }
}

impl<'a> DelimiterParser<'a> {
    pub fn new(tokens: &'a [Token], 
        ignored_tokens: &'a HashSet<usize>
    ) -> Self {
        DelimiterParser {
            base: BaseParser::new(tokens),
            ignored_tokens,
            run_vector: DelimiterRunVector::new(),
            match_vector: DelimiterMatchVector::new(),
        }
    }

    pub fn run_vector(&self) -> &DelimiterRunVector {
        &self.run_vector
    }

    pub fn match_vector(&self) -> &DelimiterMatchVector {
        &self.match_vector
    }

    fn ignored_token(&self, index: usize) -> bool {
        self.ignored_tokens.contains(&index)
    }

    fn new_run_vector(&mut self, line: usize, emphasis_count: &mut u8) {
        if *emphasis_count as usize == 0 { return; }

        let error_message = if line > 0 
            { &format!("Please check line {}", line) } else 
            {"Sorry, input text appears to have some syntax problem"};

        let prev_token = self.base.previous().expect(error_message);

        self.run_vector.push(
            DelimiterRun { token_type: prev_token.token_type(),
                token_index: self.base.current() - *emphasis_count as usize,
                byte_index: prev_token.logical_byte_end(),
                count: *emphasis_count,
                remaining: *emphasis_count,
                ..Default::default()}
        );

        *emphasis_count = 0;
    }

    /// From the input [`Token`] list, populates `self.run_vector`, which is 
    /// [`DelimiterRunVector`].
    /// 
    /// At this stage, [`DelimiterRun`] entries have `can_open` and `can_close` 
    /// remain at the default value of `false`.
    pub fn pre_process(&mut self) -> ParseResult<()> {
        self.base.save();
        let mut emphasis_count: u8 = 0;

        while !self.base.is_at_end() {
            let (token_type, line) = match self.base.peek() {
                None => break,
                Some(token) => (token.token_type(), token.line()),
            };

            if self.ignored_token(self.base.current()) {
                self.base.advance();
                continue;
            }

            match token_type {
                TokenType::Eof => break,
                TokenType::Star => emphasis_count += 1,
                TokenType::Whitespace | TokenType::Hash | TokenType::Dash | 
                TokenType::Bang | TokenType::EscapedChar | TokenType::Text | 
                TokenType::Newline | TokenType::LBracket | TokenType::RBracket | 
                TokenType::LParen | TokenType::RParen 
                    => self.new_run_vector(line, &mut emphasis_count), 
                _ => {}
            }

            self.base.advance();
        }

        self.new_run_vector(0, &mut emphasis_count);

        self.base.restore()?;
        Ok(())
    }

    /// Given an index to the [`Token`] list -- which identifies the first 
    /// emphasis of a [`DelimiterRun`], identifies and returns the previous 
    /// character and the next character of the run.
    /// 
    /// For example, in `***a *b* c***`, for first `***` opener, the token index 
    /// to the first emphasis (`*`) is `0`; similarly for the last `***` closer,
    /// the token index to the first emphasis (`*`) is `8`.
    /// 
    /// The first `***` opener previous character is a space (32, ' '), the next 
    /// character is `a`.
    /// 
    /// The last `***` closer previous character is `c`, the next character is 
    /// `¶` ([`END_OF_INPUT_CHAR`]).
    /// 
    /// # Arguments
    /// 
    /// * `tokens` — the [`Token`] list.
    /// 
    /// * `token_index` — index to a token in `tokens`.
    /// * `count` — [`DelimiterRun`]'s `count`.
    /// 
    /// # Return
    /// 
    /// `(char, char)` — on success. [`DelimiterRun`]'s previous and next characters.
    /// 
    /// [`std::error::Error`] — if some error occurs.
    fn get_flanking_prev_and_next_char(tokens: &[Token],
        token_index: usize,
        count: usize
    ) -> ParseResult<(char, char)> {
        let prev: char = match token_index {
            0 => ' ',
            _ => {  
                let token = &tokens[token_index - 1];

                token.lexeme().chars()
                    .last()
                    .ok_or("Delimiter can_open can't extract preceeding \
                        lexeme last character")?                
            }
        };

        // +count is safe: it should at most index the last element in the 
        // Token list.
        let next = match tokens.get(token_index + count) {
            // '¶' is not alphanumeric, and is not a whitespace. It is treated 
            //     as a punctuation.
            None => END_OF_INPUT_CHAR,
            Some(token) => {
                // TokenType::Newline -- 366
                if matches!(token.token_type(), TokenType::Eof | TokenType::Newline) {
                    END_OF_INPUT_CHAR
                } else {
                    token.lexeme().chars()
                        .next()
                        .ok_or("Delimiter can_open can't extract following lexeme \
                            first character")?
                }
            }
        };

        Ok((prev, next))
    }

    /// For each [`DelimiterRun`] in `self.run_vector`, attempts to set `can_open` 
    /// and `can_close`.
    /// 
    ///  # Return
    /// 
    /// * `()`  — on success.
    /// 
    /// * [`std::error::Error`] — if some error occurs.
    pub fn classify_can_open_close(&mut self) -> ParseResult<()> {
        let tokens = self.base.tokens();

        for run in self.run_vector.iter_mut() {
            let (prev_char, next_char) =
                Self::get_flanking_prev_and_next_char(tokens, run.token_index, run.count as usize)?;            

            // Left‑flanking → can_open:
            // next is NOT whitespace
            // AND
            // (
            //     next is NOT punctuation
            //     OR
            //     prev is whitespace OR punctuation
            // )  
            run.can_open = !next_char.is_whitespace() && 
                (
                    !self.base.is_punctuation(next_char) || 
                    prev_char.is_whitespace() || self.base.is_punctuation(prev_char)
                );                      

            // Right‑flanking → can_close:
            // prev is NOT whitespace
            // AND
            // (
            //     prev is NOT punctuation
            //     OR
            //     next is whitespace OR punctuation
            // )
            run.can_close = !prev_char.is_whitespace() &&
                (
                    !self.base.is_punctuation(prev_char) || 
                    next_char.is_whitespace() || self.base.is_punctuation(next_char)
                );
        }

        Ok(())
    }

    /// CommonMark rule-of-three check for emphasis delimiters.
    ///
    /// Rejects matches where:
    ///     (opener + closer) % 3 == 0
    /// and neither delimiter length is itself divisible by 3,
    /// when one of the delimiters can both open and close.
    fn satisfies_rule_of_three(&self, opener: &DelimiterRun, closer: &DelimiterRun) -> bool {
        if !(opener.can_open && opener.can_close) && !(closer.can_open && closer.can_close) {
            return true;
        }

        let sum_multiple_of_3 =
            (opener.remaining + closer.remaining) % 3 == 0;

        let not_both_multiples_of_3 =
            opener.remaining % 3 != 0 ||
            closer.remaining % 3 != 0;

        return !(sum_multiple_of_3 && not_both_multiples_of_3);
    }

    /// Pair up [`DelimiterRun`]s based on `can_open` and `can_close`, 
    /// a match would produce [`DelimiterMatch`]s and populate these 
    /// to `self.match_vector`.
    /// 
    /// Note, for some Markdown, e.g. `a*"b"*c` would not result in any 
    /// [`DelimiterMatch`] aka matched pair.
    ///
    /// # Return
    /// 
    /// * `()`  — on success.
    /// 
    /// * [`std::error::Error`] — if some error occurs.
    pub fn match_delimiters(&mut self) -> ParseResult<()> {
        let mut stack: Vec<usize> = Vec::new();

        let len = self.run_vector.len();        
        for i in 0..len {
            if !self.run_vector[i].can_close && self.run_vector[i].can_open {
                stack.push(i);
                continue;
            } 

            // `self.run_vector[i].can_close` is `true`.
            let mut closer_count = self.run_vector[i].remaining;

            // A closer run may perform multiple reductions, but each reduction must 
            // independently re-run backward opener search on the mutated stack. 
            // Consider: `***a **b* c**`, whose runs are:
            // 
            //     0: *** open
            //     1: **  open
            //     2: *   close
            //     3: **  close
            //
            // without this loop, run `3: **` would never be fully consumed.
            while closer_count > 0 && stack.len() > 0 {
                let mut stack_len = stack.len();                
                while stack_len > 0 {
                    stack_len -= 1;

                    let opener_index = stack[stack_len];
                    if (self.run_vector[i].token_type != self.run_vector[opener_index].token_type) || 
                    !self.satisfies_rule_of_three(&self.run_vector[opener_index], 
                        &self.run_vector[i]) {
                        // Exhausted stack search. No more opener to match, stop 
                        // processing this closer: move to the next run.
                        // Please note, this is semantically "no compatible opener 
                        // found", not "stack exhausted".
                        if stack_len == 0 {
                            closer_count = 0;
                        }
                        continue;
                    }

                    let mut opener_count = self.run_vector[opener_index].remaining;
                    let use_count = if opener_count >= 2 && closer_count >= 2 
                        { 2 } else { 1 };

                    self.match_vector.push(DelimiterMatch::new(opener_index, i, use_count));

                    opener_count -= use_count;
                    closer_count -= use_count;

                    self.run_vector[opener_index].remaining = opener_count;
                    self.run_vector[i].remaining = closer_count;

                    if opener_count == 0 {
                        // Remove the fully consumed opener.
                        stack.remove(stack_len);
                    }

                    break;
                }
            }

            // `**Úc*Đại*Lợi**`: the first `*` open/close gets pushed.
            if self.run_vector[i].can_open() && self.run_vector[i].remaining > 0 {
                stack.push(i);
            }
        }

        // Sort by `opener_index` ascending, then by `closer_index` descending, 
        // then `use_count` descending.
        // Nested spans need order awareness. E.g. `***xy* z**`. 
        self.match_vector.sort_by(|a, b| {
            a.opener_index.cmp(&b.opener_index) // Ascending `opener_index`
                .then(b.closer_index.cmp(&a.closer_index)) // Descending `closer_index`
                .then(b.use_count.cmp(&a.use_count)) // Descending `use_count`            
        });

        Ok(())
    }

    /// Not all [`DelimiterRun`]s are consumed, given a [`DelimiterMatch`] 
    /// match, identifies the consumed delimeters byte ranges for either 
    /// the opener or the closer [`DelimiterRun`].
    /// 
    /// # Arguments
    /// 
    /// * `delimiter_match`: a [`DelimiterMatch`] whose closer/opener 
    ///   consumed delimeter byte ranges are to be identified. 
    /// 
    /// * `is_opener`: the [`DelimiterMatch`]'s opener or closer.
    /// 
    /// # Return
    /// 
    /// `(usize, usize)` — start and end byte indexes for a [`DelimiterRun`], 
    /// depending on its role (opening/closing) in the [`DelimiterMatch`].
    /// 
    /// The end byte index is exclusive, while start byte index in inclusive.
    fn get_match_start_end_byte_range(&self, 
        delimiter_match: &DelimiterMatch,
        is_opener: bool,
    ) -> (usize, usize) {
        let run = if is_opener 
            { &self.run_vector[delimiter_match.opener_index] }
            else { &self.run_vector[delimiter_match.closer_index] };

        let run_count = run.count as usize;
        let run_remaining = run.remaining as usize;

        let start: usize = if is_opener 
            { run.byte_index - run_count + run_remaining } 
            else { run.byte_index - run_count };

        let end: usize = if is_opener 
            { run.byte_index } else { run.byte_index - run_remaining };

        (start, end)
    }    

    /// From the internal [`DelimiterMatchVector`] — `self.match_vector` — identifies 
    /// the consumed delimeter byte ranges for all [`DelimiterMatch`]s' opener and 
    /// closer [`DelimiterRun`]s.
    /// 
    /// # Return
    /// 
    /// [`Vec<(usize, usize)>`] — the vector of all byte ranges for consumed delimeters.
    fn get_delimiter_byte_ranges(&self) -> Vec<(usize, usize)> {
        let mut delimiter_byte_ranges: Vec<(usize, usize)> = Vec::new();

        for m in &self.match_vector {
            delimiter_byte_ranges.push(
                self.get_match_start_end_byte_range(m, true));

            delimiter_byte_ranges.push(
                self.get_match_start_end_byte_range(m, false));            
        }

        delimiter_byte_ranges
    }

    /// Create the clean text presentation of the input Markdown from the [`Token`]
    /// vector `self.base.tokens()`, the [`DelimiterRun`] vector `self.run_vector`, 
    /// and the [`DelimiterMatch`] vector `self.match_vector`.
    /// 
    /// # Assumption
    /// 
    /// Delimiter tokens are one byte each.
    /// 
    /// # TO_DO: Improve `delimiter_byte_ranges.iter().any(...)` Efficiency
    /// 
    /// `delimiter_byte_ranges.iter().any(...)` is not ideal: O(number_of_ranges) 
    /// for every token or O(tokens × matches).
    /// 
    /// Optimization Possibility
    /// 
    ///     1. sort the ranges
    ///     2. merge overlapping ranges
    ///     3. walk tokens and ranges together
    /// 
    /// Then it becomes: O(tokens + ranges).
    /// 
    /// # Return
    /// 
    /// * [`String`]  — on success. Clean text representation of the input Markdown.
    /// 
    /// * [`std::error::Error`] — if some error occurs.
    pub fn map_markdown_to_clean(&self) -> ParseResult<String> {
        // The byte ranges of all consumed emphases.
        let delimiter_byte_ranges = self.get_delimiter_byte_ranges();

        // Clean text, assembled from the `Token` list.
        let mut clean_text = String::new();
        for (index, token) in self.base.tokens().iter().enumerate() {
            if self.ignored_token(index) {
                continue;
            }
            
            let byte = token.logical_byte_start();

            let consumed = delimiter_byte_ranges.iter()
                .any(|(start, end)| byte >= *start && byte < *end);

            if !consumed {
                clean_text.push_str(token.lexeme());
            }
        }

        Ok(clean_text)
    }

    /// For each of the [`DelimiterRun`] in `self.run_vector`, calculates and  
    /// set the `text_index` field based on the documentation outlined in the 
    /// `struct`'s documentation.
    fn calculate_and_set_text_index(&mut self) {
        // The total bytes of all ignored `Token`s in front of the 
        // `Token` indexed by `token_index`.
        let get_ignored_tokens_byte_count = 
            |token_index: usize| -> usize {
            let mut result: usize = 0;
            for ignored_token in self.ignored_tokens.iter() {
                if *ignored_token < token_index {
                    result += self.base.tokens()[*ignored_token].lexeme().len();
                }
            };

            result
        };

        // Construct a sorted and unique list of `closer_index` for entries 
        // `self.match_vector`.
        let closer_indexes: HashSet<_> = self.match_vector
            .iter()
            .map(|m| m.closer_index)
            .collect();

        // As mentioned in the `BaseParser` module documentation, the main `Parser` 
        // passes a slice of the master `Token` vector into this parser. Therefore, 
        // the first `Token` in this slice— slice_token0 —is not necessary the first 
        // `Token` from the the master `Token` vector, slice_token0.logical_byte_start() 
        // is the byte index of the master input Markdown, for this inline, it must 
        // be substracted away from every `Token`'s `logical_byte_start()` in the token 
        // slice to correctly index to the actual text that the token slice represents.
        //
        // For example: `![***Mount Fuji*...`, the token slice passed to this parser 
        // contains `Token` for `***Mount Fuji*...` only, where `.logical_byte_start()` 
        // for the first `*` `Token` is `2`, because of `![` in front. However, being 
        // independent of the image block, it is no longer `2`, it is now `0`, every 
        // other `Token` that follows also have their `.logical_byte_start()` reduced 
        // by `2`.
        //
        // Normalise source byte offsets so the first token in this inline slice
        // is treated as starting at byte 0 in the produced `InlineContent`.
        let mut running_count = self.base.tokens()
            .first()
            .map(|t| t.logical_byte_start())
            .unwrap_or(0);

        for (index, run) in self.run_vector.iter_mut().enumerate() {
            // Is this run a closer?
            running_count += if closer_indexes.contains(&index) { run.count as usize }
                else { run.count as usize - run.remaining as usize };

            run.text_index = run.byte_index - running_count - 
                get_ignored_tokens_byte_count(run.token_index);
        }
    }

    /// Generate [`Span`]s for each of the [`DelimiterMatch`] pairs in 
    /// `self.match_vector`, using their respective [`DelimiterRun`]'s `text_index` 
    /// values as byte ranges.
    /// 
    /// # Note
    /// 
    /// The [`Span`] vector is not an "exact span construction ordering". 
    /// That is, it cannot be used as is to render HTML: it is PDF-centric,
    /// used to style PDF text.
    /// 
    /// For HTML generation, see `src/render/html.rs`.
    /// 
    /// # Return
    /// 
    /// * [`Vec<Span>`]  — on success. Clean text's emphasis information.
    /// 
    /// * [`std::error::Error`] — if some error occurs.
    pub fn produce_spans(&mut self) -> ParseResult<Vec<Span>> {
        let mut spans: Vec<Span> = Vec::new();

        self.calculate_and_set_text_index();

        for m in &self.match_vector {
            spans.push(Span::new(
                self.run_vector[m.opener_index].text_index,
                self.run_vector[m.closer_index].text_index,
                m.use_count));
        }

        Ok(spans)
    }

    /// This is the **single API** to the [`DelimiterParser`] inline parser.
    /// 
    /// ```rust
    /// let mut scanner = Scanner::new("*****Hello*world****");
    ///
    /// let res = scanner.scan_tokens();
    /// assert!(res.is_ok(), "Scanning should be valid");
    /// 
    /// let tokens = res.unwrap();
    /// let mut parser = DelimiterParser::new(&tokens);
    /// 
    /// let res = parser.parse_inline();
    /// assert!(res.is_ok(), "parse_inline() should be valid");
    /// 
    /// println!("{:?}", &res.unwrap());
    /// ```
    /// 
    /// # Return
    /// 
    /// * [`InlineContent`]  — on success. Clean text's and [`Span`] vector represents
    ///   emphasis byte ranges and style.
    /// 
    /// * [`std::error::Error`] — if some error occurs.
    pub fn parse_inline(&mut self) -> ParseResult<InlineContent> {
        self.pre_process()?;
        self.classify_can_open_close()?;
        self.match_delimiters()?;

        let text = self.map_markdown_to_clean()?;
        let spans = self.produce_spans()?;

        Ok(InlineContent::new(text, spans))
    }

}
