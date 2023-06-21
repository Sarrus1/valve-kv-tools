import { useState, useRef } from "react";
import { FormatterConfig, lintKeyvalue } from "valve_kv_tools";
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
    const errorMarkers: Monaco.editor.IMarkerData[] = lintResults.map((e) => {
      return {
        startLineNumber: e.range.start.line,
        startColumn: e.range.start.character,
        endLineNumber: e.range.end.line,
        endColumn: e.range.end.character,
        message: e.message,
        severity: Monaco.MarkerSeverity.Error,
      };
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
