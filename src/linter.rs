use std::collections::HashMap;

use pest::error::LineColLocation;

#[cfg(target_arch = "wasm32")]
use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

use crate::{KeyValue, Position, Range, Value};

#[wasm_bindgen]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum KvErrorKind {
    #[default]
    SyntaxError,
    DuplicateError,
}

/// Representation of a KeyValue linter error
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct KvError {
    /// Range of the error
    pub range: Range,

    /// Ranges that are related to the error
    /// For example, the range of duplicate entries in a duplicate error
    pub additional_ranges: Vec<Range>,

    /// Error message of the error
    pub message: String,

    /// Kind of the error
    pub kind: KvErrorKind,
}

#[cfg(target_arch = "wasm32")]
impl KvError {
    pub(crate) fn to_js(&self) -> KvErrorJs {
        KvErrorJs {
            range: self.range,
            additional_ranges: self
                .additional_ranges
                .clone()
                .into_iter()
                .map(JsValue::from)
                .collect(),
            message: self.message.clone(),
            kind: self.kind,
        }
    }
}

/// Representation of a KeyValue linter error
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = KvError, getter_with_clone)]
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct KvErrorJs {
    /// Range of the error
    #[wasm_bindgen(readonly)]
    pub range: Range,

    /// Ranges that are related to the error
    /// For example, the range of duplicate entries in a duplicate error
    #[wasm_bindgen(js_name = additionalRanges, readonly)]
    pub additional_ranges: Array,

    /// Error message of the error
    #[wasm_bindgen(readonly)]
    pub message: String,

    /// Kind of the error
    #[wasm_bindgen(readonly)]
    pub kind: KvErrorKind,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl KvErrorJs {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }
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
                        character: pos.1 as u32,
                    },
                    end: Position {
                        line: pos.0 as u32,
                        character: pos.1 as u32,
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
            errors.push(KvError {
                range,
                additional_ranges: vec![],
                message: err.variant.message().to_string(),
                kind: KvErrorKind::SyntaxError,
            });
        }
        Ok(kv) => {
            let mut dups = vec![];
            search_for_duplicates(&mut dups, &[kv]);
            for dup in dups {
                errors.push(KvError {
                    range: dup.original_declaration,
                    additional_ranges: dup.duplicate_declarations,
                    message: format!("Duplicate entry for key \"{}\"", dup.key),
                    kind: KvErrorKind::DuplicateError,
                });
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

fn search_for_duplicates(dups: &mut Vec<Duplicate>, keyvalues: &[KeyValue]) {
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
            Value::Section(section_val) => {
                if let Some(dup) = keys.get_mut(&kv.key) {
                    dup.duplicate_declarations.push(kv.key_range);
                } else {
                    keys.insert(kv.key.clone(), Duplicate::new(kv));
                }
                search_for_duplicates(dups, section_val)
            }
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
