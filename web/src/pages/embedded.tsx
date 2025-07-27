import { createSignal, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import * as game from '../bevy/draw-bevy';


import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';
import App, { NetworkDelegate } from '../organisms/app';
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
    on_peer_disconnected: null
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
}

const Embedded: Component = () => {
    const [searchParams, setSearchParams] = useSearchParams();

    return (
        <App instance_id={searchParams.id as string} delegate={embedded_delegate} />
    );
};

export default Embedded;
