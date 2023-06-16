import { Config } from "./interfaces";

export function makeDefaultSettings(): Config {
  return {
    use_tabs: false,
    indent_size: 4,
    max_empty_lines: 1,
  };
}
