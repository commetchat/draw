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


class GameUtils {
  get_system_time(): number {
    let time = new Date().getTime();
    return time / 1000;
  };

  get_random_u32(): number {
    let u32_max = 4_294_967_294
    let r = Math.random() * u32_max
    return Math.round(r);
  }
}

declare global {
  interface Window { gameDatabase: WebDatabase; gameUtils: GameUtils }
}

async function initGame(instance_id: string, delegate: NetworkDelegate) {
  window.gameDatabase = new WebDatabase(instance_id, delegate)
  window.gameUtils = new GameUtils()

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
  send_to: (message: Uint8Array, to: string) => void;
  on_received: ((message: Uint8Array, from: string) => void) | null;
  on_peer_connected: ((from: string) => void) | null;
  on_peer_disconnected: ((from: string) => void) | null;
}

interface AppProps {
  instance_id: string,
  delegate: NetworkDelegate,
}

const App: Component<AppProps> = (props) => {

  initGame(props.instance_id, props.delegate);

  const save_to_file = () => {
    window.gameDatabase.save_to_file()
  }

  props.delegate.on_received = (message, from) => {

    game.web_receive_packet(from, message)
  }

  props.delegate.on_peer_connected = (from) => {
    console.log("Peer connected! ", from)
    window.gameDatabase.send_strokes_to_user(from);
  }

  props.delegate.on_peer_disconnected = (from) => {

  }

  let [getLines, setLines] = createSignal<string>("")
  let original = console.log;

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

    try {
      game.queue_ui_message(msg);
    } catch (_) {
      console.log("Failed to pass message to game!");
    }
  }

  return (
    <div class={styles.App}>

      <header>
        <div style={"width: 100vw; height: 100vh;  overflow: hidden;"}>
          <canvas style={"z-index: 1; position: relative;"} id="bevy-portal"></canvas>
        </div>


        <UI callback={handleUIMessage} />
      </header>
    </div >
  );
};

export default App;
export type { NetworkDelegate };
