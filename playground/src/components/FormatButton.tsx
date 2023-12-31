import { Button, Snackbar } from "@mui/material";
import { useState, forwardRef } from "react";
import { formatKeyvalue } from "valve_kv_tools";
import MuiAlert, { AlertProps } from "@mui/material/Alert";

import { HeaderProps } from "../interfaces";

export const Alert = forwardRef<HTMLDivElement, AlertProps>(function Alert(
  props,
  ref
) {
  return <MuiAlert elevation={6} ref={ref} variant="filled" {...props} />;
});

export function FormatButton(props: HeaderProps) {
  const [showError, setShowError] = useState(false);

  const handleClose = (
    event?: React.SyntheticEvent | Event,
    reason?: string
  ) => {
    if (reason === "clickaway") {
      return;
    }
    setShowError(false);
  };

  return (
    <>
      <Button
        color="primary"
        variant="contained"
        style={{ backgroundColor: "grey", marginLeft: "auto" }}
        onClick={(_) => {
          try {
            props.setCode(formatKeyvalue(props.code, props.settings));
          } catch (err) {
            console.log(err);
            setShowError(true);
          }
        }}
      >
        Format
      </Button>
      <Snackbar
        open={showError}
        autoHideDuration={6000}
        onClose={handleClose}
        anchorOrigin={{ vertical: "bottom", horizontal: "right" }}
      >
        <Alert onClose={handleClose} severity="error" sx={{ width: "100%" }}>
          There is an error in your syntax.
        </Alert>
      </Snackbar>
    </>
  );
}
