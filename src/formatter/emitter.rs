use super::tokens::{KvToken, TokenKind};
use crate::FormatterConfig;

use std::cmp::min;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct Emitter {
    pub(super) buffer: Vec<String>,
    config: FormatterConfig,
    indent: usize,
    current_line: String,
    pub(super) tokens: Vec<TokenKind>,

    /// Indexes of line breaks in the input string, in reverse order.
    pub(super) line_breaks: Vec<usize>,
    pub(super) line_nb: usize,
    pub(super) first_col_line: usize,
    prev_token: Option<TokenKind>,
}

impl Emitter {
    pub(super) fn new(config: FormatterConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    // Tokens should handle line breaks before they are written,
    // i.e the LBrace tokens handles the line break before it.
    pub(super) fn emit(&mut self) {
        self.tokens.reverse();
        while let Some(token_kind) = self.tokens.pop() {
            match &token_kind {
                TokenKind::Key(token) => self.emit_key(token),
                TokenKind::Value(token) => self.emit_value(token),
                TokenKind::LineComment(token) => self.emit_line_comment(token),
                TokenKind::BlockComment(token) => self.emit_block_comment(token),
                TokenKind::LBrace => {
                    self.push_line();
                    self.current_line.push('{');
                    self.push_line();
                    self.indent += 1;
                }
                TokenKind::RBrace => {
                    self.push_line();
                    self.indent -= 1;
                    self.current_line.push('}');
                }
            }
            self.prev_token = Some(token_kind.clone());
        }
        self.push_line();
    }

    fn emit_key(&mut self, token: &KvToken) {
        self.push_line();
        self.current_line.push_str(token.text.as_str());
    }

    fn emit_value(&mut self, token: &KvToken) {
        if let Some(prev_token_kind) = &self.prev_token {
            match prev_token_kind {
                TokenKind::Key(_) => {
                    self.current_line.push_str(self.indent_string().as_str());
                }
                TokenKind::LineComment(prev_token) => {
                    // This should not be possible, but just in case.
                    let diff = token.range.start.line.abs_diff(prev_token.range.start.line);
                    if diff == 0 {
                        self.current_line.push_str("  ");
                    } else {
                        for _ in 0..min(diff, self.config.max_empty_lines) {
                            self.push_line();
                        }
                    }
                }
                TokenKind::BlockComment(prev_token) => {
                    let diff = token.range.start.line.abs_diff(prev_token.range.start.line);
                    if diff == 0 {
                        self.current_line.push_str("  ");
                    } else {
                        for _ in 0..min(diff, self.config.max_empty_lines) {
                            self.push_line();
                        }
                    }
                }
                _ => (),
            }
        }
        self.current_line.push_str(token.text.as_str());
    }

    fn emit_line_comment(&mut self, token: &KvToken) {
        if let Some(prev_token_kind) = &self.prev_token {
            match prev_token_kind {
                TokenKind::Key(_) => {
                    self.current_line.push_str("  ");
                }
                TokenKind::Value(prev_token) => {
                    let diff = token.range.start.line.abs_diff(prev_token.range.start.line);
                    if diff == 0 {
                        self.current_line.push_str("  ");
                    } else {
                        for _ in 0..min(diff, self.config.max_empty_lines) {
                            self.push_line();
                        }
                    }
                }
                TokenKind::LBrace | TokenKind::RBrace => {
                    self.push_line();
                }
                TokenKind::BlockComment(prev_token) => {
                    let diff = token.range.start.line.abs_diff(prev_token.range.start.line);
                    if diff == 0 {
                        self.current_line.push_str("  ");
                    } else {
                        for _ in 0..min(diff, self.config.max_empty_lines) {
                            self.push_line();
                        }
                    }
                }
                TokenKind::LineComment(prev_token) => {
                    let diff = token.range.start.line.abs_diff(prev_token.range.start.line);
                    if diff == 0 {
                        self.current_line.push_str("  ");
                    } else {
                        for _ in 0..min(diff, self.config.max_empty_lines) {
                            self.push_line();
                        }
                    }
                }
            }
        }
        self.current_line.push_str(token.text.as_str());
        self.push_line();
    }

    fn emit_block_comment(&mut self, token: &KvToken) {
        if let Some(prev_token_kind) = &self.prev_token {
            match prev_token_kind {
                TokenKind::Key(_) => {
                    self.current_line.push_str("  ");
                }
                TokenKind::Value(prev_token) => {
                    let diff = token.range.start.line.abs_diff(prev_token.range.start.line);
                    if diff == 0 {
                        self.current_line.push_str("  ");
                    } else {
                        for _ in 0..min(diff, self.config.max_empty_lines) {
                            self.push_line();
                        }
                    }
                }
                TokenKind::LBrace | TokenKind::RBrace => {
                    self.push_line();
                }
                TokenKind::BlockComment(prev_token) => {
                    let diff = token.range.start.line.abs_diff(prev_token.range.start.line);
                    if diff == 0 {
                        self.current_line.push_str("  ");
                    } else {
                        for _ in 0..min(diff, self.config.max_empty_lines) {
                            self.push_line();
                        }
                    }
                }
                TokenKind::LineComment(prev_token) => {
                    let diff = token.range.start.line.abs_diff(prev_token.range.start.line);
                    if diff == 0 {
                        self.current_line.push_str("  ");
                    } else {
                        for _ in 0..min(diff, self.config.max_empty_lines) {
                            self.push_line();
                        }
                    }
                }
            }
        }
        self.current_line.push_str(token.text.as_str());
    }

    fn indent_string(&self) -> String {
        if self.config.use_tabs {
            "\t".to_string()
        } else {
            " ".repeat(self.config.indent_size as usize)
        }
    }

    fn indent(&self) -> String {
        self.indent_string().repeat(self.indent)
    }

    fn push_line(&mut self) {
        if self.current_line.is_empty() {
            return;
        }
        self.buffer
            .push(format!("{}{}", self.indent(), self.current_line));
        self.current_line.clear();
    }
}
