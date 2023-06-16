use crate::Rule;

use self::emitter::Emitter;

mod collectors;
mod emitter;
mod tokens;

pub fn format_keyvalue(input: &str) -> Result<String, Box<pest::error::Error<Rule>>> {
    let mut writer = Emitter::default();
    writer.collect_tokens(input)?;
    writer.emit();

    Ok(writer.buffer.join("\n"))
}
