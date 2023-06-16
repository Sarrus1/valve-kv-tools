use valve_kv_tools::{format_keyvalue, FormatterConfig};

#[test]
fn formatter_key_value() {
    let input = r#""key"    "value""#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section() {
    let input = r#""key"
{
    "key"    "value"
}"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
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
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
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
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_value_prefix_comment() {
    let input = r#"/* comment */
"key"    "value""#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_value_suffix_comment() {
    let input = r#""key"    "value"  // comment"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_value_middle_comment() {
    let input = r#""key"  /* comment */  "value""#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_prefix_block_comment() {
    let input = r#"/* comment */
"key"
{
    "key"    "value"
}"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_prefix_line_comment() {
    let input = r#"// comment
"key"
{
    "key"    "value"
}"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_middle_block_comment() {
    let input = r#""key"  /* comment */
{
    "key"    "value"
}"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_middle_line_comment() {
    let input = r#""key"  // comment
{
    "key"    "value"
}"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_middle_suffix_line_comment() {
    let input = r#""key"
{
    // comment
    "key"    "value"
}"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_middle_suffix_block_comment() {
    let input = r#""key"
{
    /* comment */
    "key"    "value"
}"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_key_value_prefix_block_comment() {
    let input = r#""key"
{
    /* comment */  "key"    "value"
}"#;
    let output = r#""key"
{
    /* comment */
    "key"    "value"
}"#;
    assert_eq!(
        output,
        format_keyvalue(input, FormatterConfig::default()).unwrap()
    );
}

#[test]
fn formatter_key_section_key_value_middle_block_comment() {
    let input = r#""key"
{
    "key"  /* comment */  "value"
}"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_end_prefix_line_comment() {
    let input = r#""key"
{
    "key"    "value"  // comment
}"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_end_prefix_block_comment() {
    let input = r#""key"
{
    "key"    "value"  /* comment */
}"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_key_section_suffix_block_comment() {
    let input = r#""key"
{
    "key"    "value"
}
/* comment */"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}

#[test]
fn formatter_multiple_comments() {
    let input = r#""key"
{
    "key"    "value"
    // comment
    // comment
    /* comment */
    // comment
}
/* comment */"#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}
