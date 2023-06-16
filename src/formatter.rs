use crate::Rule;

use self::emitter::Emitter;

mod collectors;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormatterConfig {
    /// Should the formatter use tabs or spaces for indentation.
    pub use_tabs: bool,

    /// Number of tabs or spaces to use per indent level.
    pub indent_size: u32,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            use_tabs: false,
            indent_size: 4,
        }
    }
}
