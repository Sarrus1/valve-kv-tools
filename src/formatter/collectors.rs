use pest::{iterators::Pair, Parser, Span};

use crate::{KeyValueParser, Position, Range, Rule};

use super::{
    emitter::Emitter,
    tokens::{KvToken, TokenKind},
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct RangeCollector {
    /// Indexes of line breaks in the input string, in reverse order.
    pub(super) line_breaks: Vec<usize>,
    pub(super) line_nb: usize,
    pub(super) first_col_line: usize,
}

impl RangeCollector {
    pub(crate) fn collect_linebreaks(&mut self, input: &str) {
        for (i, c) in input.chars().enumerate() {
            if c == '\n' {
                self.line_breaks.push(i);
            }
        }
        self.line_breaks.reverse();
    }

    pub(crate) fn span_to_range(&mut self, span: Span) -> Range {
        let mut line_nb = self.line_nb;
        let mut first_col_line = self.first_col_line;
        while let Some(line_break) = self.line_breaks.last() {
            if *line_break >= span.end() {
                break;
            }
            if *line_break < span.start() {
                first_col_line = *line_break + 1;
                line_nb += 1;
            }
            self.first_col_line = *line_break + 1;
            self.line_nb += 1;
            self.line_breaks.pop();
        }

        Range {
            start: Position {
                line: line_nb as u32,
                character: (span.start() - first_col_line) as u32,
            },
            end: Position {
                line: self.line_nb as u32,
                character: (span.end() - self.first_col_line) as u32,
            },
        }
    }
}

impl Emitter {
    pub(super) fn collect_tokens(
        &mut self,
        input: &str,
    ) -> Result<(), Box<pest::error::Error<Rule>>> {
        self.range_collector.collect_linebreaks(input);
        let pairs = KeyValueParser::parse(Rule::start, input)?;

        for pair in pairs {
            if let Rule::start = pair.as_rule() {
                for sub_pair in pair.into_inner() {
                    match sub_pair.as_rule() {
                        Rule::keyvalue => self.collect_keyvalue(sub_pair),
                        Rule::COMMENT => self.collect_comment(sub_pair),
                        Rule::EOI => (),
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
                        range: self.range_collector.span_to_range(sub_pair.as_span()),
                    };
                    self.tokens.push(TokenKind::Key(token))
                }
                Rule::value => {
                    let token = KvToken {
                        text: sub_pair.as_str().to_string(),
                        range: self.range_collector.span_to_range(sub_pair.as_span()),
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
            range: self.range_collector.span_to_range(pair.as_span()),
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
