import { FormatterConfig } from "valve_kv_tools";

export interface HeaderProps {
  readonly code: string;
  settings: FormatterConfig;
  setCode: React.Dispatch<React.SetStateAction<string>>;
}

export interface SettingsPanelProps {
  settings: FormatterConfig;
  setSettings: React.Dispatch<React.SetStateAction<FormatterConfig>>;
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
