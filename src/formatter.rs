use pest::{iterators::Pair, Parser};

use crate::{KeyValue, KeyValueParser, Rule, Value};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Writer {
    indent: usize,
    buffer: Vec<String>,
    current_line: String,
    line_break_indexes: Vec<usize>,
}

impl Writer {
    fn write_keyvalue(&mut self, pair: Pair<Rule>) {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::key => {
                    self.current_line.push_str(pair.as_str());
                }
                Rule::value => {
                    self.current_line.push_str("    ");
                    self.current_line.push_str(pair.as_str());
                    self.push_line();
                }
                Rule::section => {
                    self.write_section(pair);
                }
                _ => (),
            }
        }
    }

    fn write_section(&mut self, pair: Pair<Rule>) {
        self.push_line();
        self.current_line.push('{');
        self.push_line();
        self.indent += 1;
        for pair in pair.into_inner() {
            if let Rule::keyvalue = pair.as_rule() {
                self.write_keyvalue(pair);
            }
        }
        self.indent -= 1;
        self.current_line.push('}');
        self.push_line();
    }

    fn find_line_breaks(&mut self, input: &str) {
        for (i, c) in input.chars().enumerate() {
            if c == '\n' {
                self.line_break_indexes.push(i);
            }
        }
    }

    fn indent(&self) -> String {
        "  ".repeat(self.indent)
    }

    fn push_line(&mut self) {
        self.buffer
            .push(format!("{}{}", self.indent(), self.current_line));
        self.current_line.clear();
    }
}

pub fn format_keyvalue(input: &str) -> Result<String, Box<pest::error::Error<Rule>>> {
    let mut writer = Writer::default();
    let pairs = KeyValueParser::parse(Rule::start, input)?;

    for pair in pairs {
        if let Rule::start = pair.as_rule() {
            for pair in pair.into_inner() {
                if let Rule::keyvalue = pair.as_rule() {
                    writer.write_keyvalue(pair);
                }
            }
        }
    }

    Ok(writer.buffer.join("\n"))
}
