import { createSignal, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import * as game from '../bevy/draw-bevy';


import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';
import { WebDatabase } from '../web_database';
import UI from './ui';
import { UIMessage } from '../bindings/ui_binding';


declare global {
  interface Window { gameDatabase: WebDatabase; }
}

async function initGame(instance_id: string) {
  window.gameDatabase = new WebDatabase(instance_id)

  game.default();
}

function openFile() {
  var input = document.createElement('input');
  input.type = 'file';

  input.onchange = e => {
    // getting a hold of the file reference
    var file = (e.target! as any).files[0];

    // setting up the reader
    var reader = new FileReader();
    reader.readAsArrayBuffer(file);

    // here we tell the reader what to do when it's done reading...
    reader.onload = readerEvent => {
      var content = (readerEvent as any).target.result; // this is the content!

      game.load_file(new Uint8Array(content))
    }
  }

  input.click();
}

interface NetworkDelegate {
  send_to: (message: Uint8Array, to: String) => void;
  on_received: ((message: Uint8Array, from: String) => void) | null;
  on_peer_connected: ((from: String) => void) | null;
  on_peer_disconnected: ((from: String) => void) | null;
}

interface AppProps {
  instance_id: string,
  delegate: NetworkDelegate,
}

const App: Component<AppProps> = (props) => {

  initGame(props.instance_id);

  props.delegate.on_received = (message, from) => {

  }

  props.delegate.on_peer_connected = (from) => {

  }

  props.delegate.on_peer_disconnected = (from) => {

  }

  let [getLines, setLines] = createSignal<string>("")
  let original = console.log;

  console.log = (e) => {
    original(e);

    let s = getLines();
    setLines((`${e}\n` + s).substring(0, 2000));
  }

  const save_to_file = () => {
    window.gameDatabase.save_to_file()
  }



  function handleUIMessage(message: UIMessage): void {

    if (message.type == "LoadFile") {
      openFile()
      return;
    }

    if (message.type == "SaveFile") {
      save_to_file()
      return;
    }

    let msg = JSON.stringify(message);
    console.log("Passing UI Message to Game: " + msg,);
  }

  return (
    <div class={styles.App}>

      <header>
        <div style={"width: 100vw; height: 100vh;  overflow: hidden;"}>
          <canvas style={"z-index: 1; position: relative;"} id="bevy-portal"></canvas>
        </div>

        <div class='pointer-events-none text-white text-xs ' style={"z-index: 2; position: absolute; top: 0; left: 0; width: 100%; height: 100%"}>
          <div style={"position: absolute; right: 0; top: 0;"}>
            <textarea class='pointer-events-auto' style={"width: 75vw; height: 20vh; "} value={getLines()} disabled>
            </textarea>
          </div>
        </div>


        <UI callback={handleUIMessage} />
      </header>
    </div >
  );
};

export default App;
export type { NetworkDelegate };
