import { createSignal, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import * as game from '../bevy/draw-bevy';


import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';
import App, { GameDelegate, NetworkDelegate } from '../organisms/app';
import { useSearchParams } from '@solidjs/router';


let embedded_delegate: NetworkDelegate = {
    send_to: function (message: Uint8Array, to: String): void {
        window.parent.postMessage({
            "type": "send_to",
            "info": { "to": to },
            "body": message
        });
    },

    on_received: null,
    on_peer_connected: null,
    on_peer_disconnected: null,
    on_ready: null,
    upload_chunk: null,
    download_chunks: function (chunk_id: string): void {
        throw new Error('Function not implemented.');
    },
    broadcast: function (message: Uint8Array): void {
        throw new Error('Function not implemented.');
    }
}

window.onmessage = (message) => {
    if (message.data.type == "peerconnect") {
        let from = message.data.info.from as string;
        embedded_delegate.on_peer_connected!(from)
    }

    if (message.data.type == "recv_from") {
        let from = message.data.info.from as string;
        embedded_delegate.on_received!(message.data.body, from)
    }

    if (message.data.type == "ready") {
        console.log("Received ready message!");
        console.log(message);
        let id = message.data.info.id as string;
        embedded_delegate.on_ready!(id);
    }
}

let game_delegate: GameDelegate = {
    on_ready: function (): void {
    }
}

const Embedded: Component = () => {
    const [searchParams, setSearchParams] = useSearchParams();

    return (
        <App instance_id={searchParams.id as string} network_delegate={embedded_delegate} game_delegate={game_delegate} />
    );
};

export default Embedded;
