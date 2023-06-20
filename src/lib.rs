pub(crate) mod formatter;
mod linter;
mod serializer;

pub use {self::formatter::*, self::linter::*, self::serializer::*};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = formatKeyvalue)]
pub fn format_keyvalue(input: String, config: JsValue) -> Result<String, JsValue> {
    let config: FormatterConfig = serde_wasm_bindgen::from_value(config)?;

    Ok(formatter::format_keyvalue(input.as_str(), config).unwrap())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = lintKeyvalue)]
pub fn lint_keyvalue(input: String) -> JsValue {
    serde_wasm_bindgen::to_value(&linter::lint_keyvalue(input.as_str())).unwrap()
}
