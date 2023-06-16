mod formatter;
mod serializer;

pub use {self::formatter::*, self::serializer::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn format_keyvalue(input: String, config: JsValue) -> Result<String, JsValue> {
    let config: FormatterConfig = serde_wasm_bindgen::from_value(config)?;

    Ok(formatter::format_keyvalue(input.as_str(), config).unwrap())
}
