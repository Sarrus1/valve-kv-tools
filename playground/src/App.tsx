import { useState, useRef } from "react";
import { Config } from "./interfaces";
import Editor from "@monaco-editor/react";
import Header from "./components/Header";
import SettingsPanel from "./components/SettingsPanel";
import "./App.css";
import { defaultCode } from "./text";
import { makeDefaultSettings } from "./utils";

function App() {
  const [code, setCode] = useState(defaultCode);
  const [settings, setSettings] = useState<Config>(makeDefaultSettings());

  const editorRef = useRef(null);

  function handleEditorDidMount(editor: any, monaco: any) {
    editorRef.current = editor;
  }

  function handleEditorChange(value: string | undefined, event: any) {
    if (value !== undefined) {
      setCode(value);
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
