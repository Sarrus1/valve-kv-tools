use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "valve-kv.pest"]
struct KeyValueParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValue {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(String),
    Section(Vec<KeyValue>),
}

pub fn parse_keyvalue(input: &str) -> Result<KeyValue, Box<pest::error::Error<Rule>>> {
    let pairs = KeyValueParser::parse(Rule::start, input)?;
    let mut kv = KeyValue {
        key: String::new(),
        value: Value::String(String::new()),
    };

    for pair in pairs {
        if let Rule::start = pair.as_rule() {
            for pair in pair.into_inner() {
                if let Rule::keyvalue = pair.as_rule() {
                    kv = serialize_kv(pair);
                }
            }
        }
    }

    Ok(kv)
}

fn serialize_kv(pair: Pair<Rule>) -> KeyValue {
    let mut kv = KeyValue {
        key: String::new(),
        value: Value::String(String::new()),
    };
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::key => {
                kv.key = pair.as_str().to_string().replace('"', "");
            }
            Rule::value => {
                kv.value = Value::String(pair.as_str().to_string().replace('"', ""));
            }
            Rule::section => {
                kv.value = Value::Section(serialize_section(pair));
            }
            _ => (),
        }
    }

    kv
}

fn serialize_section(pair: Pair<Rule>) -> Vec<KeyValue> {
    let mut kvs = Vec::new();
    for pair in pair.into_inner() {
        if let Rule::keyvalue = pair.as_rule() {
            kvs.push(serialize_kv(pair));
        }
    }

    kvs
}
