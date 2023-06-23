use wasm_bindgen::prelude::wasm_bindgen;

/// Position in a text document expressed as zero-based line and character offset.
/// A position is between two characters like an 'insert' cursor in a editor.
#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Default)]
pub struct Position {
    /// Line position in a document (zero-based).
    #[wasm_bindgen(readonly)]
    pub line: u32,

    /// Character offset on a line in a document (zero-based).
    #[wasm_bindgen(readonly)]
    pub character: u32,
}

#[wasm_bindgen]
impl Position {
    #[wasm_bindgen(constructor)]
    pub fn new(line: u32, character: u32) -> Position {
        Position { line, character }
    }
}

/// A range in a text document expressed as (zero-based) start and end positions.
/// A range is comparable to a selection in an editor. Therefore the end position is exclusive.
#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct Range {
    /// The range's start position.
    #[wasm_bindgen(readonly)]
    pub start: Position,

    /// The range's end position.
    #[wasm_bindgen(readonly)]
    pub end: Position,
}

#[wasm_bindgen]
impl Range {
    #[wasm_bindgen(constructor)]
    pub fn new(start: Position, end: Position) -> Range {
        Range { start, end }
    }
}

#[test]
fn test_range() {
    let range = Range::new(Position::new(1, 2), Position::new(3, 4));
    assert_eq!(range.start.line, 1);
    assert_eq!(range.start.character, 2);
    assert_eq!(range.end.line, 3);
    assert_eq!(range.end.character, 4);
}
