use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{collectors::RangeCollector, Range};

#[derive(Parser)]
#[grammar = "valve-kv.pest"]
pub struct KeyValueParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValue {
    pub key: String,
    pub value: Value,
    pub key_range: Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(String),
    Section(Vec<KeyValue>),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Serializer {
    range_collector: RangeCollector,
}

pub fn serialize_keyvalue(input: &str) -> Result<KeyValue, Box<pest::error::Error<Rule>>> {
    let mut serializer = Serializer::default();
    serializer.range_collector.collect_linebreaks(&input);

    let pairs = KeyValueParser::parse(Rule::start, input)?;
    let mut kv = KeyValue {
        key: String::new(),
        value: Value::String(String::new()),
        key_range: Range::default(),
    };

    for pair in pairs {
        if let Rule::start = pair.as_rule() {
            for pair in pair.into_inner() {
                if let Rule::keyvalue = pair.as_rule() {
                    kv = serializer.serialize_kv(pair);
                }
            }
        }
    }

    Ok(kv)
}

impl Serializer {
    fn serialize_kv(&mut self, pair: Pair<Rule>) -> KeyValue {
        let mut kv = KeyValue {
            key: String::new(),
            value: Value::String(String::new()),
            key_range: Range::default(),
        };
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::key => {
                    kv.key = pair.as_str().to_string().replace('"', "");
                    kv.key_range = self.range_collector.span_to_range(pair.as_span());
                }
                Rule::value => {
                    kv.value = Value::String(pair.as_str().to_string().replace('"', ""));
                }
                Rule::section => {
                    kv.value = Value::Section(self.serialize_section(pair));
                }
                _ => (),
            }
        }

        kv
    }

    fn serialize_section(&mut self, pair: Pair<Rule>) -> Vec<KeyValue> {
        let mut kvs = Vec::new();
        for pair in pair.into_inner() {
            if let Rule::keyvalue = pair.as_rule() {
                kvs.push(self.serialize_kv(pair));
            }
        }

        kvs
    }
}
