<div align="center">
  <h1><code>Valve KV Tools</code></h1>
  <p>
    <strong>Serializer, Formatter and Linter for Valve's KeyValue format built with <a href="https://crates.io/crates/pest">Pest</a></strong>
  </p>
  <p style="margin-bottom: 0.5ex;">
    <a href="https://crates.io/crates/valve_kv_tools">
      <img alt="Crates.io" src="https://img.shields.io/crates/d/valve-kv-tools">
    </a>
    <a href="https://crates.io/crates/valve_kv_tools">
      <img alt="Crates.io" src="https://img.shields.io/crates/v/valve-kv-tools">
    </a>
    <a href="https://github.com/Sarrus1/valve-kv-tools/actions/workflows/release.yml">
      <img
        alt="Github release status"
        src="https://github.com/Sarrus1/valve-kv-tools/actions/workflows/release.yml/badge.svg"
      />
    </a>
    <a href="https://codecov.io/gh/Sarrus1/valve-kv-tools" > 
      <img
        alt="Code coverage"
        src="https://codecov.io/gh/Sarrus1/valve-kv-tools/branch/main/graph/badge.svg?token=5T6QQZYPQ6"/> 
    </a>
    <img alt="GitHub" src="https://img.shields.io/github/license/Sarrus1/valve-kv-tools">
  </p>
</div>


# Example

```rust
use valve_kv_tools::{format_keyvalue, FormatterConfig};

fn main() {
    let input = r#""key"    "value""#;
    let output = format_keyvalue(input, FormatterConfig::default()).unwrap();
    assert_eq!(input, output);
}
```