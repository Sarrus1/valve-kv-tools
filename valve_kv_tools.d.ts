export as namespace valve_kv_tools;

export interface FormatterConfig {
  /**
   * Should the formatter use tabs or spaces for indentation.
   */
  use_tabs: boolean;

  /**
   * Number of tabs or spaces to use per indent level.
   */
  indent_size: number;

  /**
   * Maximum number of consecutive empty lines.
   */
  max_empty_lines: number;
}

export interface Position {
  /**
   * Line position in a document (zero-based).
   */
  line: number;

  /**
   * Character offset on a line in a document (zero-based).
   */
  character: number;
}

interface Range {
  /**
   * The range's start position.
   */
  start: Position;

  /**
   * The range's end position.
   */
  end: Position;
}

export interface KvError {
  /**
   * Range of the error.
   */
  range: Range;

  /**
   * Ranges that are related to the error.
   * For example, the range of duplicate entries in a duplicate error.
   */
  additional_ranges: Range[];

  /**
   * Error message of the error.
   */
  message: string;
}

/**
 * Format a string of keyvalue.
 * @param {string} input Input string for the formatter.
 * @param {FormatterConfig} config Config object that specifies formatter configuration.
 * @returns {string} Formatted output.
 * @throws Invalid input error.
 */
export function formatKeyvalue(input: string, config: FormatterConfig): string;

/**
 * Lint a string of keyvalue.
 * @param {string} input Input string for the linter.
 * @returns {KvError[]} Array of errors that the linter encountered.
 */
export function lintKeyvalue(input: string): KvError[];
