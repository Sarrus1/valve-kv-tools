import { useState, useRef } from "react";
import { Config, ErrorMarker, KvError } from "./interfaces";
import Editor, { Monaco } from "@monaco-editor/react";
import Header from "./components/Header";
import SettingsPanel from "./components/SettingsPanel";
import "./App.css";
import { defaultCode } from "./text";
import { makeDefaultSettings } from "./utils";
import { lint_keyvalue } from "valve_kv_tools";

function App() {
  const [code, setCode] = useState(defaultCode);
  const [settings, setSettings] = useState<Config>(makeDefaultSettings());

  const editorRef = useRef<any>(null);
  const modelRef = useRef<any>(null);

  function handleEditorDidMount(editor: any, monaco: Monaco) {
    editorRef.current = editor;
    modelRef.current = editor.getModel();
  }

  function handleEditorChange(value: string | undefined, event: any) {
    if (value !== undefined) {
      setCode(value);
      const lint_results: KvError[] = lint_keyvalue(value);
      console.log(lint_results);
      const errorMarkers: ErrorMarker[] = lint_results.map((e) => {
        return {
          startLineNumber: e.range.start.line,
          startColumn: e.range.start.character,
          endLineNumber: e.range.end.line,
          endColumn: e.range.end.character,
          message: e.message,
        };
      });
      editorRef.current?.setModelMarkers(
        modelRef.current,
        "error",
        errorMarkers
      );
    }
  }

  return (
    <div style={{ overflowX: "hidden" }}>
      <Header code={code} settings={settings} setCode={setCode} />
      <div className="grid grid-cols-2">
        <SettingsPanel settings={settings} setSettings={setSettings} />
        <Editor
          height="93.3vh"
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
