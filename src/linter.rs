use std::collections::HashMap;

use lsp_types::{Position, Range};
use pest::error::LineColLocation;
use serde::{Deserialize, Serialize};

use crate::{KeyValue, Value};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParsingError {
    range: Range,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DuplicateError {
    range: Range,
    original_declaration_range: Range,
    message: String,
}

pub enum KvError {
    ParsingError(ParsingError),
    DuplicateError(DuplicateError),
}

pub fn lint_keyvalue(input: &str) -> Vec<KvError> {
    let mut errors = vec![];
    let serialized = super::serialize_keyvalue(input);
    match serialized {
        Err(err) => {
            let range = match err.line_col {
                LineColLocation::Pos(pos) => Range {
                    start: Position {
                        line: pos.0 as u32,
                        character: 0,
                    },
                    end: Position {
                        line: pos.1 as u32,
                        character: 0,
                    },
                },
                LineColLocation::Span(start, end) => Range {
                    start: Position {
                        line: start.0 as u32,
                        character: start.1 as u32,
                    },
                    end: Position {
                        line: end.0 as u32,
                        character: end.1 as u32,
                    },
                },
            };
            errors.push(KvError::ParsingError(ParsingError {
                range,
                message: err.variant.message().to_string(),
            }));
        }
        Ok(kv) => {
            let mut dups = vec![];
            search_for_duplicates(&mut dups, &vec![kv]);
            for dup in dups {
                for dup_range in dup.duplicate_declarations {
                    errors.push(KvError::DuplicateError(DuplicateError {
                        range: dup_range,
                        original_declaration_range: dup.original_declaration,
                        message: format!("Duplicate entry for key \"{}\"", dup.key),
                    }))
                }
            }
        }
    }

    errors
}

struct Duplicate {
    key: String,
    original_declaration: Range,
    duplicate_declarations: Vec<Range>,
}

impl Duplicate {
    fn new(keyvalue: &KeyValue) -> Self {
        Self {
            key: keyvalue.key.clone(),
            original_declaration: keyvalue.key_range,
            duplicate_declarations: vec![],
        }
    }
}

fn search_for_duplicates(dups: &mut Vec<Duplicate>, keyvalues: &Vec<KeyValue>) {
    let mut keys: HashMap<String, Duplicate> = HashMap::default();
    for kv in keyvalues.iter() {
        match &kv.value {
            Value::String(_) => {
                if let Some(dup) = keys.get_mut(&kv.key) {
                    dup.duplicate_declarations.push(kv.key_range);
                } else {
                    keys.insert(kv.key.clone(), Duplicate::new(kv));
                }
            }
            Value::Section(section_val) => search_for_duplicates(dups, section_val),
        }
    }
    dups.extend(keys.into_iter().filter_map(|(_, dup)| {
        if !dup.duplicate_declarations.is_empty() {
            Some(dup)
        } else {
            None
        }
    }));
}
