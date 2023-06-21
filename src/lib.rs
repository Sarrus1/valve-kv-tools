mod common;
pub(crate) mod formatter;
mod linter;
mod serializer;

pub use {self::common::*, self::formatter::*, self::linter::*, self::serializer::*};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Format a string of keyvalue.
///
/// @param {string} input Input string for the formatter.
/// @param {FormatterConfig} config Config object that specifies formatter configuration.
/// @returns {string} Formatted output.
/// @throws Invalid input error.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = formatKeyvalue, skip_jsdoc)]
pub fn format_keyvalue(input: String, config: FormatterConfig) -> Result<String, JsValue> {
    Ok(formatter::format_keyvalue(input.as_str(), config).unwrap())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<KvError>")]
    pub type KvErrorArray;
}

/// Lint a string of keyvalue.
///
/// @param {string} input Input string for the linter.
///@returns {Array<KvError>} Array of errors that the linter encountered.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = lintKeyvalue, skip_jsdoc)]
pub fn lint_keyvalue(input: String) -> KvErrorArray {
    use js_sys::Array;
    use wasm_bindgen::JsCast;

    let tmp: Array = linter::lint_keyvalue(input.as_str())
        .into_iter()
        .map(|e| e.to_js())
        .map(JsValue::from)
        .collect();
    tmp.unchecked_into::<KvErrorArray>()
}
