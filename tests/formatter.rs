use valve_kv_tools::format_keyvalue;

#[test]
fn formatter_key_value() {
    let input = r#""key"    "value""#;
    let output = format_keyvalue(input).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section() {
    let input = r#""key"
{
  "key"    "value"
}"#;
    let output = format_keyvalue(input).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_multiple() {
    let input = r#""key"
{
  "key"    "value"
  "key"    "value"
  "key"    "value"
}"#;
    let output = format_keyvalue(input).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_nested() {
    let input = r#""key"
{
  "key"    "value"
  "key"
  {
    "key"    "value"
  }
  "key"    "value"
}"#;
    let output = format_keyvalue(input).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_value_suffix_comment() {
    let input = r#""key"    "value"  // comment"#;
    let output = format_keyvalue(input).unwrap();
    assert_eq!(input, output);
}
