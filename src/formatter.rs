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
        let mut pair_iter = pair.into_inner().peekable();
        while let Some(sub_pair) = pair_iter.next() {
            match sub_pair.as_rule() {
                Rule::key => {
                    self.current_line.push_str(sub_pair.as_str());
                }
                Rule::value => {
                    self.current_line.push_str("    ");
                    self.current_line.push_str(sub_pair.as_str());
                }
                Rule::section => {
                    self.write_section(sub_pair);
                }
                Rule::COMMENT => self.write_comment(sub_pair),
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
                self.push_line();
            }
        }
        self.indent -= 1;
        self.current_line.push('}');
    }

    fn write_comment(&mut self, pair: Pair<Rule>) {
        self.current_line.push_str(pair.as_str());
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
            let mut pair_iter = pair.into_inner().peekable();
            while let Some(sub_pair) = pair_iter.next() {
                match sub_pair.as_rule() {
                    Rule::keyvalue => {
                        writer.write_keyvalue(sub_pair);
                        if let Some(next_pair) = pair_iter.peek() {
                            if let Rule::COMMENT = next_pair.as_rule() {
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
