import { TextField, Switch, FormGroup, FormControlLabel } from "@mui/material";
import {
  SettingRowBoolProps,
  SettingRowNumericProps,
  SettingsPanelProps,
} from "../interfaces";

function SettingRowNumeric(props: SettingRowNumericProps) {
  return (
    <div
      className="items-center grid gap-4 mb-1"
      style={{ gridTemplateColumns: "1fr 15rem" }}
    >
      <span className="font-roboto font-bold">{props.name}</span>
      <TextField
        className="col-span-1"
        inputProps={{ inputMode: "numeric", pattern: "[0-9]*" }}
        defaultValue={props.defaultValue}
        variant="outlined"
        size="small"
        onChange={props.onChange}
      />
    </div>
  );
}

function SettingRowBool(props: SettingRowBoolProps) {
  return (
    <FormGroup>
      <FormControlLabel
        control={<Switch defaultChecked onChange={props.onChange} />}
        label={props.name}
      />
    </FormGroup>
  );
}

function SettingsPanel(props: SettingsPanelProps) {
  return (
    <div style={{ margin: "1rem" }}>
      <SettingRowBool
        name="Should the formatter use tabs or spaces for indentation."
        onChange={(e) => {
          props.settings.use_tabs = Boolean(e.target.value);
        }}
      />
      <SettingRowNumeric
        name="Number of tabs or spaces to use per indent level."
        onChange={(e) => {
          props.settings.indent_size = Number(e.target.value);
        }}
        defaultValue={4}
      />
      <SettingRowNumeric
        name="Breaks before enum"
        onChange={(e) => {
          props.settings.max_empty_lines = Number(e.target.value);
        }}
        defaultValue={1}
      />
    </div>
  );
}

export default SettingsPanel;
