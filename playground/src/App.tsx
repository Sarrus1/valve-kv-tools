import { useState, useRef } from "react";
import {
  FormatterConfig,
  KvErrorKind,
  Range,
  lintKeyvalue,
} from "valve_kv_tools";
import Editor, { OnMount, OnChange } from "@monaco-editor/react";
import * as Monaco from "monaco-editor";

import Header from "./components/Header";
import SettingsPanel from "./components/SettingsPanel";
import "./App.css";
import { defaultCode } from "./text";
import { makeDefaultSettings } from "./utils";

function App() {
  const [code, setCode] = useState(defaultCode);
  const [settings, setSettings] = useState<FormatterConfig>(
    makeDefaultSettings()
  );

  const editorRef = useRef<typeof Monaco.editor | null>(null);
  const modelRef = useRef<Monaco.editor.ITextModel | null>(null);

  const handleEditorDidMount: OnMount = (editor, monaco) => {
    editorRef.current = monaco.editor;
    modelRef.current = editor.getModel();
  };

  const handleEditorChange: OnChange = (value, _) => {
    if (value === undefined || modelRef.current === null) {
      return;
    }
    setCode(value);
    const lintResults = lintKeyvalue(value);
    let errorMarkers: Monaco.editor.IMarkerData[] = [];
    lintResults.forEach((e) => {
      let severity;
      switch (e.kind) {
        case KvErrorKind.SyntaxError:
          severity = Monaco.MarkerSeverity.Error;
          break;
        case KvErrorKind.DuplicateError:
          severity = Monaco.MarkerSeverity.Warning;
      }
      errorMarkers.push({
        startLineNumber: e.range.start.line + 1,
        startColumn: e.range.start.character + 1,
        endLineNumber: e.range.end.line + 1,
        endColumn: e.range.end.character + 1,
        message: e.message,
        severity,
      });
      if (e.kind === KvErrorKind.DuplicateError) {
        e.additionalRanges.forEach((dup: Range) =>
          errorMarkers.push({
            startLineNumber: dup.start.line + 1,
            startColumn: dup.start.character + 1,
            endLineNumber: dup.end.line + 1,
            endColumn: dup.end.character + 1,
            message: e.message,
            severity: Monaco.MarkerSeverity.Hint,
          })
        );
      }
    });
    editorRef.current?.setModelMarkers(modelRef.current, "error", errorMarkers);
  };

  return (
    <div style={{ overflowX: "hidden" }}>
      <Header code={code} settings={settings} setCode={setCode} />
      <div className="grid grid-cols-2">
        <SettingsPanel settings={settings} setSettings={setSettings} />
        <Editor
          height="100vh"
          width="50vw"
          theme="vs-dark"
          defaultLanguage="cpp"
          value={code}
          onChange={handleEditorChange}
          onMount={handleEditorDidMount}
        />
      </div>
    </div>
  );
}

export default App;
