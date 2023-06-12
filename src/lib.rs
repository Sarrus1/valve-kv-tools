mod formatter;
mod serializer;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub use {self::formatter::*, self::serializer::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn format_keyvalue(input: String) -> Result<String, JsValue> {
    Ok(formatter::format_keyvalue(input.as_str()).unwrap())
}
