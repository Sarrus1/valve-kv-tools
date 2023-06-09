use crate::{KeyValueParser, Rule};
use pest::{iterators::Pair, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Writer {
    indent: usize,
    buffer: Vec<String>,
    current_line: String,
}

impl Writer {
    fn write_keyvalue(&mut self, pair: Pair<Rule>) {
        let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();
        let mut pairs_iter = pairs.iter().peekable();
        while let Some(sub_pair) = pairs_iter.next() {
            match sub_pair.as_rule() {
                Rule::key => {
                    self.current_line.push_str(sub_pair.as_str());
                    if let Some(next) = pairs_iter.peek() {
                        match next.as_rule() {
                            Rule::COMMENT => {
                                if let Some(next) = pairs_iter.next() {
                                    self.current_line.push_str("  ");
                                    self.current_line.push_str(next.as_str());
                                }
                                if let Some(next) = pairs_iter.peek() {
                                    if next.as_rule() == Rule::value {
                                        self.current_line.push_str("  ");
                                    }
                                }
                            }
                            Rule::section => (),
                            Rule::value => {
                                self.current_line.push_str("    ");
                            }
                            _ => (),
                        }
                    }
                }
                Rule::value => {
                    self.current_line.push_str(sub_pair.as_str());
                    if let Some(next) = pairs_iter.peek() {
                        match next.as_rule() {
                            Rule::COMMENT => {
                                self.current_line.push_str("  ");
                                self.current_line.push_str(next.as_str());
                            }
                            _ => println!("unhandled rule: {:?}", next.as_rule()),
                        }
                    }
                }
                Rule::section => {
                    self.write_section(sub_pair.clone());
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
        let mut pairs_iter = pair.into_inner().peekable();
        while let Some(sub_pair) = pairs_iter.next() {
            match sub_pair.as_rule() {
                Rule::keyvalue => {
                    self.write_keyvalue(sub_pair);
                    if let Some(next) = pairs_iter.peek() {
                        if next.as_rule() == Rule::COMMENT {
                            // FIXME: This does not work for multiple comments.
                            if let Some(next) = pairs_iter.next() {
                                self.current_line.push_str("  ");
                                self.current_line.push_str(next.as_str());
                            }
                            if let Some(next) = pairs_iter.peek() {
                                if next.as_rule() == Rule::keyvalue {
                                    self.current_line.push_str("  ");
                                }
                            }
                        }
                    }
                    self.push_line();
                }
                Rule::COMMENT => {
                    // This only happens between keyvalues.
                    self.write_comment(sub_pair);
                    self.push_line();
                }
                _ => (),
            }
        }
        self.indent -= 1;
        self.current_line.push('}');
        self.push_line();
    }

    fn write_comment(&mut self, pair: Pair<Rule>) {
        self.current_line.push_str(pair.as_str());
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
    let pairs = KeyValueParser::parse(Rule::start, input)?;

    for pair in pairs {
        if let Rule::start = pair.as_rule() {
            let mut pair_iter = pair.into_inner().peekable();
            while let Some(sub_pair) = pair_iter.next() {
                match sub_pair.as_rule() {
                    Rule::keyvalue => {
                        writer.write_keyvalue(sub_pair);
                        if let Some(next_pair) = pair_iter.peek() {
                            if let Rule::COMMENT = next_pair.as_rule() {
                                if let Some(last) = writer.buffer.last() {
                                    if last.ends_with('}') {
                                        writer.push_line();
                                        continue;
                                    }
                                }
                                writer.current_line.push_str("  ");
                                continue;
                            }
                        }
                        writer.push_line();
                    }
                    Rule::COMMENT => {
                        writer.write_comment(sub_pair);
                        writer.push_line();
                    }
                    _ => println!("unhandled rule: {:?}", sub_pair.as_rule()),
                }
            }
        }
    }

    Ok(writer.buffer.join("\n"))
}
