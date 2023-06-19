export interface Config {
  /* Should the formatter use tabs or spaces for indentation. */
  use_tabs: boolean;

  /* Number of tabs or spaces to use per indent level. */
  indent_size: number;

  /* Maximum number of consecutive empty lines. */
  max_empty_lines: number;
}

export interface HeaderProps {
  readonly code: string;
  settings: Config;
  setCode: React.Dispatch<React.SetStateAction<string>>;
}

export interface SettingsPanelProps {
  settings: Config;
  setSettings: React.Dispatch<React.SetStateAction<Config>>;
}

export interface SettingRowBoolProps {
  name: string;
  onChange: (
    event: React.ChangeEvent<HTMLInputElement>,
    checked: boolean
  ) => void;
}

export interface SettingRowNumericProps {
  name: string;
  onChange: (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => void;
  defaultValue: number;
}

export interface Range {
  start: Position;
  end: Position;
}

export interface Position {
  line: number;
  character: number;
}

export interface KvError {
  message: string;
  range: Range;
  additional_ranges: Range[];
}

export interface ErrorMarker {
  startLineNumber: number;
  startColumn: number;
  endLineNumber: number;
  endColumn: number;
  message: string;
}
