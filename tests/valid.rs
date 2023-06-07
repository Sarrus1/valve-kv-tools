use valve_kv_tools::{parse_keyvalue, KeyValue, Value};

#[test]
fn single_key_value() {
    let input = r#""key" "value""#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn single_key_value_suffix_whitespace() {
    let input = r#""key" "value"
    "#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn single_key_value_prefix_whitespace() {
    let input = r#"
    "key" "value""#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn single_key_value_multiple_lines() {
    let input = r#""key"
    "value""#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn single_key_value_prefix_block_comment() {
    let input = r#"/* comment */ "key"
    "value""#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn single_key_value_middle_block_comment() {
    let input = r#""key" /* comment */ "value""#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn single_key_value_suffix_block_comment() {
    let input = r#""key" "value" /* comment */"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn single_key_value_suffix_line_comment() {
    let input = r#""key" "value" // comment"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(kv.value, Value::String("value".to_string()));
}

#[test]
fn single_key_section() {
    let input = r#""key1" {"key2" "value"}"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
        }])
    );
}

#[test]
fn single_key_section_multiple_lines() {
    let input = r#""key1"
  {
    "key2" "value"
  }"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
        }])
    );
}

#[test]
fn single_key_section_block_comment_1() {
    let input = r#"/* comment */ "key1"
  {
    "key2" "value"
  }"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
        }])
    );
}

#[test]
fn single_key_section_block_comment_2() {
    let input = r#""key1" /* comment */
  {
    "key2" "value"
  }"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
        }])
    );
}

#[test]
fn single_key_section_block_comment_3() {
    let input = r#""key1"
  { /* comment */
    "key2" "value"
  }"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
        }])
    );
}

#[test]
fn single_key_section_block_comment_4() {
    let input = r#""key1"
  {
    /* comment */ "key2" "value"
  }"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
        }])
    );
}

#[test]
fn single_key_section_block_comment_5() {
    let input = r#""key1"
  {
   "key2" "value"
  /* comment */}"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
        }])
    );
}

#[test]
fn single_key_section_block_comment_6() {
    let input = r#""key1"
  {
   "key2" "value"
  } /* comment */"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![KeyValue {
            key: "key2".to_string(),
            value: Value::String("value".to_string()),
        }])
    );
}

#[test]
fn single_key_section_multiple_key_values() {
    let input = r#""key1"
  {
    "key2" "value"
    "key3" "value"
  }"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![
            KeyValue {
                key: "key2".to_string(),
                value: Value::String("value".to_string()),
            },
            KeyValue {
                key: "key3".to_string(),
                value: Value::String("value".to_string()),
            }
        ])
    );
}

#[test]
fn single_key_section_nested() {
    let input = r#""key1"
  {
    "key2" "value"
    "key3" {
      "key4" "value"
    }
  }"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key1");
    assert_eq!(
        kv.value,
        Value::Section(vec![
            KeyValue {
                key: "key2".to_string(),
                value: Value::String("value".to_string()),
            },
            KeyValue {
                key: "key3".to_string(),
                value: Value::Section(vec![KeyValue {
                    key: "key4".to_string(),
                    value: Value::String("value".to_string()),
                }]),
            }
        ])
    );
}

#[test]
fn single_key_section_repeated_keys() {
    let input = r#""key"
  {
    "key" "value"
    "key" {
      "key" "value"
    }
  }"#;
    let kv = parse_keyvalue(input).unwrap();
    assert_eq!(kv.key, "key");
    assert_eq!(
        kv.value,
        Value::Section(vec![
            KeyValue {
                key: "key".to_string(),
                value: Value::String("value".to_string()),
            },
            KeyValue {
                key: "key".to_string(),
                value: Value::Section(vec![KeyValue {
                    key: "key".to_string(),
                    value: Value::String("value".to_string()),
                }]),
            }
        ])
    );
}
