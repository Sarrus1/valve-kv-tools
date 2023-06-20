import { FormatterConfig } from "valve_kv_tools";

export function makeDefaultSettings(): FormatterConfig {
  return {
    use_tabs: false,
    indent_size: 4,
    max_empty_lines: 1,
  };
}
