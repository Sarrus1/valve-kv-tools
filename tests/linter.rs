use valve_kv_tools::{lint_keyvalue, KvError, KvErrorKind, Position, Range};

#[test]
fn linter_keyvalue_ok() {
    let input = r#""key"    "value""#;
    assert_eq!(lint_keyvalue(input), []);
}

#[test]
fn linter_keyvalue_section_ok() {
    let input = r#""key"
{
    "key"    "value"
}"#;
    assert_eq!(lint_keyvalue(input), []);
}

#[test]
fn linter_keyvalue_section_duplicate() {
    let input = r#""key"
{
    "key"    "value"
    "key"    "value"
}"#;
    assert_eq!(
        lint_keyvalue(input),
        [KvError {
            range: Range {
                start: Position {
                    line: 2,
                    character: 4,
                },
                end: Position {
                    line: 2,
                    character: 9,
                },
            },
            additional_ranges: vec![Range {
                start: Position {
                    line: 3,
                    character: 4,
                },
                end: Position {
                    line: 3,
                    character: 9,
                },
            },],
            message: "Duplicate entry for key \"key\"".to_string(),
            kind: KvErrorKind::DuplicateError,
        }]
    );
}

#[test]
fn linter_keyvalue_section_duplicate_2() {
    let input = r#""key"
{
    "key"
    {
        "key"    "value"
    }
    "key"    "value"
}"#;
    assert_eq!(
        lint_keyvalue(input),
        [KvError {
            range: Range {
                start: Position {
                    line: 2,
                    character: 4,
                },
                end: Position {
                    line: 2,
                    character: 9,
                },
            },
            additional_ranges: vec![Range {
                start: Position {
                    line: 6,
                    character: 4,
                },
                end: Position {
                    line: 6,
                    character: 9,
                },
            }],
            message: "Duplicate entry for key \"key\"".to_string(),
            kind: KvErrorKind::DuplicateError,
        }]
    );
}

#[test]
fn linter_keyvalue_section_duplicate_3() {
    let input = r#""key"
{
    "key"
    {
        "key"    "value"
    }
    "key"
    {
        "key"    "value"
    }
}"#;
    assert_eq!(
        lint_keyvalue(input),
        [KvError {
            range: Range {
                start: Position {
                    line: 2,
                    character: 4,
                },
                end: Position {
                    line: 2,
                    character: 9,
                },
            },
            additional_ranges: vec![Range {
                start: Position {
                    line: 6,
                    character: 4,
                },
                end: Position {
                    line: 6,
                    character: 9,
                },
            }],
            message: "Duplicate entry for key \"key\"".to_string(),
            kind: KvErrorKind::DuplicateError,
        }]
    );
}

#[test]
fn linter_syntax_error_1() {
    let input = r#""key"
{"#;
    assert_eq!(
        lint_keyvalue(input),
        [KvError {
            range: Range {
                start: Position {
                    line: 2,
                    character: 2
                },
                end: Position {
                    line: 2,
                    character: 2
                }
            },
            additional_ranges: vec![],
            message: "expected COMMENT, r_brace, or string".to_string(),
            kind: KvErrorKind::SyntaxError
        }]
    )
}

#[test]
fn linter_syntax_error_2() {
    let input = r#""key" "val"#;
    assert_eq!(
        lint_keyvalue(input),
        [KvError {
            range: Range {
                start: Position {
                    line: 1,
                    character: 7
                },
                end: Position {
                    line: 1,
                    character: 7
                }
            },
            additional_ranges: vec![],
            message: "expected COMMENT, l_brace, or string".to_string(),
            kind: KvErrorKind::SyntaxError
        }]
    )
}
