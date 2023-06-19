use lsp_types::{Position, Range};
use valve_kv_tools::{serialize_keyvalue, KeyValue, Value};

#[test]
fn serialize_value() {
    let input = r#""key" "value""#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn serialize_value_suffix_whitespace() {
    let input = r#""key" "value"
    "#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn serialize_value_prefix_whitespace() {
    let input = r#"
    "key" "value""#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn serialize_value_multiple_lines() {
    let input = r#""key"
    "value""#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn serialize_value_prefix_block_comment() {
    let input = r#"/* comment */ "key"
    "value""#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn serialize_value_middle_block_comment() {
    let input = r#""key" /* comment */ "value""#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn serialize_value_suffix_block_comment() {
    let input = r#""key" "value" /* comment */"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn serialize_value_suffix_line_comment() {
    let input = r#""key" "value" // comment"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn serialize_section() {
    let input = r#""key1" {"key2" "value"}"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
            key_range: Range {
                start: Position::new(0, 8),
                end: Position::new(0, 14),
            }
        }])
    );
}

#[test]
fn serialize_section_multiple_lines() {
    let input = r#""key1"
  {
    "key2" "value"
  }"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
            key_range: Range {
                start: Position::new(2, 4),
                end: Position::new(2, 10),
            }
        }])
    );
}

#[test]
fn serialize_section_block_comment_1() {
    let input = r#"/* comment */ "key1"
  {
    "key2" "value"
  }"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
            key_range: Range {
                start: Position::new(2, 4),
                end: Position::new(2, 10),
            }
        }])
    );
}

#[test]
fn serialize_section_block_comment_2() {
    let input = r#""key1" /* comment */
  {
    "key2" "value"
  }"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
            key_range: Range {
                start: Position::new(2, 4),
                end: Position::new(2, 10),
            }
        }])
    );
}

#[test]
fn serialize_section_block_comment_3() {
    let input = r#""key1"
  { /* comment */
    "key2" "value"
  }"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
            key_range: Range {
                start: Position::new(2, 4),
                end: Position::new(2, 10),
            }
        }])
    );
}

#[test]
fn serialize_section_block_comment_4() {
    let input = r#""key1"
  {
    /* comment */ "key2" "value"
  }"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
            key_range: Range {
                start: Position::new(2, 18),
                end: Position::new(2, 24),
            }
        }])
    );
}

#[test]
fn serialize_section_block_comment_5() {
    let input = r#""key1"
  {
   "key2" "value"
  /* comment */}"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
            key_range: Range {
                start: Position::new(2, 3),
                end: Position::new(2, 9),
            }
        }])
    );
}

#[test]
fn serialize_section_block_comment_6() {
    let input = r#""key1"
  {
   "key2" "value"
  } /* comment */"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
            key_range: Range {
                start: Position::new(2, 3),
                end: Position::new(2, 9),
            }
        }])
    );
}

#[test]
fn serialize_section_multiple_key_values() {
    let input = r#""key1"
  {
    "key2" "value"
    "key3" "value"
  }"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![
            KeyValue {
                key: "key2".to_string(),
                value: Value::String("value".to_string()),
                key_range: Range {
                    start: Position::new(2, 4),
                    end: Position::new(2, 10),
                }
            },
            KeyValue {
                key: "key3".to_string(),
                value: Value::String("value".to_string()),
                key_range: Range {
                    start: Position::new(3, 4),
                    end: Position::new(3, 10),
                }
            }
        ])
    );
}

#[test]
fn serialize_section_nested() {
    let input = r#""key1"
  {
    "key2" "value"
    "key3" {
      "key4" "value"
    }
  }"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![
            KeyValue {
                key: "key2".to_string(),
                value: Value::String("value".to_string()),
                key_range: Range {
                    start: Position::new(2, 4),
                    end: Position::new(2, 10),
                }
            },
            KeyValue {
                key: "key3".to_string(),
                value: Value::Section(vec![KeyValue {
                    key: "key4".to_string(),
                    value: Value::String("value".to_string()),
                    key_range: Range {
                        start: Position::new(4, 6),
                        end: Position::new(4, 12),
                    }
                }]),
                key_range: Range {
                    start: Position::new(3, 4),
                    end: Position::new(3, 10),
                }
            }
        ])
    );
}

#[test]
fn serialize_section_repeated_keys() {
    let input = r#""key"
  {
    "key" "value"
    "key" {
      "key" "value"
    }
  }"#;
    let kv = serialize_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(
        kv.value,
        Value::Section(vec![
            KeyValue {
                key: "key".to_string(),
                value: Value::String("value".to_string()),
                key_range: Range {
                    start: Position::new(2, 4),
                    end: Position::new(2, 9),
                }
            },
            KeyValue {
                key: "key".to_string(),
                value: Value::Section(vec![KeyValue {
                    key: "key".to_string(),
                    value: Value::String("value".to_string()),
                    key_range: Range {
                        start: Position::new(4, 6),
                        end: Position::new(4, 11),
                    }
                }]),
                key_range: Range {
                    start: Position::new(3, 4),
                    end: Position::new(3, 9),
                }
            }
        ])
    );
}
