use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use self::emitter::Emitter;
use crate::Rule;

pub(crate) mod collectors;
mod emitter;
mod tokens;

pub fn format_keyvalue(
    input: &str,
    config: FormatterConfig,
) -> Result<String, Box<pest::error::Error<Rule>>> {
    let mut emitter = Emitter::new(config);
    emitter.collect_tokens(input)?;
    emitter.emit();

    Ok(emitter.buffer.join("\n"))
}

/// Configuration options for the formatter
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormatterConfig {
    /// Should the formatter use tabs or spaces for indentation
    #[wasm_bindgen(js_name = useTabs)]
    pub use_tabs: bool,

    /// Number of tabs or spaces to use per indent level
    #[wasm_bindgen(js_name = indentSize)]
    pub indent_size: u32,

    /// Maximum number of consecutive empty lines
    #[wasm_bindgen(js_name = maxEmptyLines)]
    pub max_empty_lines: u32,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            use_tabs: false,
            indent_size: 4,
            max_empty_lines: 1,
        }
    }
}

#[test]
fn test_formatter_config() {
    let mut config = FormatterConfig::default();
    assert!(!config.use_tabs);
    config.use_tabs = true;
    assert!(config.use_tabs);
    assert_eq!(config.indent_size, 4);
    config.indent_size = 2;
    assert_eq!(config.indent_size, 2);
    assert_eq!(config.max_empty_lines, 1);
    config.max_empty_lines = 2;
    assert_eq!(config.max_empty_lines, 2);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl FormatterConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(use_tabs: bool, indent_size: u32, max_empty_lines: u32) -> Self {
        Self {
            use_tabs,
            indent_size,
            max_empty_lines,
        }
    }
}
