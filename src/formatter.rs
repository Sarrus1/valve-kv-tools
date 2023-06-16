use crate::{KeyValueParser, Rule};
use lsp_types::{Position, Range};
use pest::{iterators::Pair, Parser, Span};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct KvToken {
    text: String,
    range: Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Key(KvToken),
    Value(KvToken),
    LineComment(KvToken),
    BlockComment(KvToken),
    LBrace,
    RBrace,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Writer {
    indent: usize,
    buffer: Vec<String>,
    current_line: String,
    tokens: Vec<TokenKind>,

    /// Indexes of line breaks in the input string, in reverse order.
    line_breaks: Vec<usize>,
    line_nb: usize,
    line_start: usize,
}

impl Writer {
    fn collect_linebreaks(&mut self, input: &str) {
        for (i, c) in input.chars().enumerate() {
            if c == '\n' {
                self.line_breaks.push(i);
            }
        }
        self.line_breaks.reverse();
    }

    fn span_to_range(&mut self, span: Span) -> Range {
        let start_line = self.line_nb;
        let line_start = span.start() - self.line_start;
        while let Some(line_break) = self.line_breaks.last() {
            if *line_break >= span.end() || *line_break < span.start() {
                break;
            }
            self.line_start = *line_break + 1;
            self.line_nb += 1;
            self.line_breaks.pop();
        }

        Range {
            start: Position {
                line: start_line as u32,
                character: line_start as u32,
            },
            end: Position {
                line: self.line_nb as u32,
                character: (span.end() - self.line_start) as u32,
            },
        }
    }

    pub(self) fn collect_tokens(
        &mut self,
        input: &str,
    ) -> Result<(), Box<pest::error::Error<Rule>>> {
        self.collect_linebreaks(input);
        let pairs = KeyValueParser::parse(Rule::start, input)?;

        for pair in pairs {
            if let Rule::start = pair.as_rule() {
                for sub_pair in pair.into_inner() {
                    match sub_pair.as_rule() {
                        Rule::keyvalue => self.collect_keyvalue(sub_pair),
                        Rule::COMMENT => self.collect_comment(sub_pair),
                        _ => eprintln!(
                            "unhandled rule in token collection: {:?}",
                            sub_pair.as_rule()
                        ),
                    }
                }
            }
        }

        Ok(())
    }

    fn collect_keyvalue(&mut self, pair: Pair<Rule>) {
        let pair_inner = pair.into_inner();
        for sub_pair in pair_inner {
            match sub_pair.as_rule() {
                Rule::key => {
                    let token = KvToken {
                        text: sub_pair.as_str().to_string(),
                        range: self.span_to_range(sub_pair.as_span()),
                    };
                    self.tokens.push(TokenKind::Key(token))
                }
                Rule::value => {
                    let token = KvToken {
                        text: sub_pair.as_str().to_string(),
                        range: self.span_to_range(sub_pair.as_span()),
                    };
                    self.tokens.push(TokenKind::Value(token))
                }
                Rule::section => self.collect_section(sub_pair),
                Rule::COMMENT => self.collect_comment(sub_pair),
                _ => eprintln!(
                    "unhandled rule in keyvalue collection: {:?}",
                    sub_pair.as_rule()
                ),
            }
        }
    }

    fn collect_comment(&mut self, pair: Pair<Rule>) {
        let token = KvToken {
            text: pair.as_str().to_string(),
            range: self.span_to_range(pair.as_span()),
        };
        if pair.as_str().starts_with("//") {
            self.tokens.push(TokenKind::LineComment(token))
        } else {
            self.tokens.push(TokenKind::BlockComment(token))
        }
    }

    fn collect_section(&mut self, pair: Pair<Rule>) {
        self.tokens.push(TokenKind::LBrace);
        let pair_inner = pair.into_inner();
        for sub_pair in pair_inner {
            match sub_pair.as_rule() {
                Rule::keyvalue => self.collect_keyvalue(sub_pair),
                Rule::COMMENT => self.collect_comment(sub_pair),
                _ => eprintln!(
                    "unhandled rule in section collection: {:?}",
                    sub_pair.as_rule()
                ),
            }
        }
        self.tokens.push(TokenKind::RBrace);
    }
}

impl Writer {
    // Tokens should handle line breaks before they are written, i.e the LBrace tokens handles the line break before it.
    fn emit(&mut self) {
        let mut prev_token: Option<TokenKind> = None;
        // FIXME: This clone is horrible.
        self.tokens.reverse();
        while let Some(token_kind) = self.tokens.pop() {
            match &token_kind {
                TokenKind::Key(token) => {
                    self.push_line();
                    self.current_line.push_str(token.text.as_str());
                }
                TokenKind::Value(token) => {
                    if let Some(prev_token_kind) = prev_token {
                        match prev_token_kind {
                            TokenKind::Key(_) => {
                                self.current_line.push_str("    ");
                            }
                            TokenKind::LineComment(prev_token) => {
                                // This should not be possible, but just in case.
                                if prev_token.range.start.line == token.range.start.line {
                                    self.current_line.push_str("  ");
                                } else {
                                    self.push_line();
                                }
                            }
                            TokenKind::BlockComment(prev_token) => {
                                if prev_token.range.start.line == token.range.start.line {
                                    self.current_line.push_str("  ");
                                } else {
                                    self.push_line();
                                }
                            }
                            _ => (),
                        }
                    }

                    self.current_line.push_str(token.text.as_str());
                }
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
                TokenKind::LineComment(token) => {
                    if let Some(prev_token_kind) = prev_token {
                        match prev_token_kind {
                            TokenKind::Key(_) => {
                                self.current_line.push_str("  ");
                            }
                            TokenKind::Value(prev_token) => {
                                if prev_token.range.start.line == token.range.start.line {
                                    self.current_line.push_str("  ");
                                } else {
                                    self.push_line();
                                }
                            }
                            TokenKind::LBrace | TokenKind::RBrace => {
                                self.push_line();
                            }
                            _ => (),
                        }
                    }
                    self.current_line.push_str(token.text.as_str());
                }
                TokenKind::BlockComment(token) => {
                    if let Some(prev_token_kind) = prev_token {
                        match prev_token_kind {
                            TokenKind::Key(_) => {
                                self.current_line.push_str("  ");
                            }
                            TokenKind::Value(prev_token) => {
                                if prev_token.range.start.line == token.range.start.line {
                                    self.current_line.push_str("  ");
                                } else {
                                    self.push_line();
                                }
                            }
                            TokenKind::LBrace | TokenKind::RBrace => {
                                self.push_line();
                            }
                            _ => (),
                        }
                    }
                    self.current_line.push_str(token.text.as_str());
                }
            }
            prev_token = Some(token_kind.clone());
        }
        self.push_line();
    }

    fn indent(&self) -> String {
        "  ".repeat(self.indent)
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

pub fn format_keyvalue(input: &str) -> Result<String, Box<pest::error::Error<Rule>>> {
    let mut writer = Writer::default();
    writer.collect_tokens(input)?;
    writer.emit();
    Ok(writer.buffer.join("\n"))
}
