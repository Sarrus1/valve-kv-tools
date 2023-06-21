import { FormatterConfig } from "valve_kv_tools";

export function makeDefaultSettings(): FormatterConfig {
  return new FormatterConfig(false, 4, 1);
}
