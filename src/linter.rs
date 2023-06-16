use lsp_types::{Position, Range};
use pest::{error::LineColLocation, Parser};
use serde::{Deserialize, Serialize};

use crate::{KeyValueParser, Rule};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KvError {
    range: Range,
    message: String,
}

pub fn lint_keyvalue(input: &str) -> Option<KvError> {
    let res = KeyValueParser::parse(Rule::start, input);
    if let Some(err) = res.err() {
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
        return Some(KvError {
            range,
            message: err.variant.message().to_string(),
        });
    }

    None
}
