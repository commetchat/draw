import { createSignal, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import * as game from '../bevy/draw-bevy';


import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';
import { WebDatabase } from '../web_database';
import UI, { UIDelegate } from './ui';
import { UIMessage } from '../bindings/ui_binding';



class GameUtils {
  constructor(delegate: NetworkDelegate, game_delegate: GameDelegate, ui_delegate: UIDelegate) {
    this.network_delegate = delegate;
    this.game_delegate = game_delegate;
    this.ui_delegate = ui_delegate;
  }

  game_delegate: GameDelegate
  network_delegate: NetworkDelegate
  ui_delegate: UIDelegate
  wake_lock: WakeLockSentinel | undefined

  // Called when bevy side is ready
  init() {
    window.gameDatabase.init();
    this.game_delegate.on_ready()

    if ("wakeLock" in navigator) {
      navigator.wakeLock.request().then((v) => {
        console.log("Acquired wake lock!")
        this.wake_lock = v;

      }).catch((v) => {
        console.log("Failed to get wake lock!");
        console.log(v);
      });
    } else {
      console.log("Wake lock not supported!");
    }
  }

  ui_message_callback(data_str: string) {
    let msg = JSON.parse(data_str) as UIMessage;
    if (this.ui_delegate.on_received_msg != null) {
      this.ui_delegate.on_received_msg!(msg);
    }
  }

  get_system_time(): number {
    let time = new Date().getTime();
    return time / 1000;
  };

  get_random_u32(): number {
    let u32_max = 4_294_967_294
    let r = Math.random() * u32_max
    return Math.round(r);
  }

  web_send_packet(to: string, data: Uint8Array) {
    this.network_delegate.send_to(data, to);
  }
}

declare global {
  interface Window { gameDatabase: WebDatabase; gameUtils: GameUtils }
}


let ui_delegate: UIDelegate = {
  on_received_msg: null
};


async function initGame(instance_id: string, network_delegate: NetworkDelegate, game_delegate: GameDelegate) {
  window.gameDatabase = new WebDatabase(instance_id, network_delegate)
  window.gameUtils = new GameUtils(network_delegate, game_delegate, ui_delegate)

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

interface GameDelegate {
  on_ready: () => void
}

interface AppProps {
  instance_id: string,
  network_delegate: NetworkDelegate,
  game_delegate: GameDelegate
}


const App: Component<AppProps> = (props) => {

  initGame(props.instance_id, props.network_delegate, props.game_delegate);

  const save_to_file = () => {
    window.gameDatabase.save_to_file()
  }

  props.network_delegate.on_received = (message, from) => {

    game.web_receive_packet(from, message)
  }

  props.network_delegate.on_peer_connected = (from) => {
    console.log("Peer connected! ", from)
    game.web_peer_connected(from);
    window.gameDatabase.send_strokes_to_user(from);

  }

  props.network_delegate.on_peer_disconnected = (from) => {
    game.web_peer_disconnected(from);
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

    try {
      game.queue_ui_message(msg);
    } catch (_) {
    }
  }

  return (
    <div class={styles.App}  >
      <div class="touch-pan-x">

        <header>
          <div class='w-lvw h-svh overflow-clip'>
            <canvas style={"z-index: 1; position: relative;"} id="bevy-portal"></canvas>
          </div>


          <UI callback={handleUIMessage} delegate={ui_delegate} />
        </header>
      </div>
    </div >
  );
};

export default App;
export type { NetworkDelegate, GameDelegate };
