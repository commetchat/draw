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
            to: to,
            message: message.buffer
        }, {
            transfer: [message.buffer]
        });
    },

    on_received: null,
    on_peer_connected: null,
    on_peer_disconnected: null
}

window.onmessage = (message) => {
    console.log(`${window.location.search} Received Message: `,)
}

const Embedded: Component = () => {
    const [searchParams, setSearchParams] = useSearchParams();

    return (
        <App instance_id={searchParams.id as string} delegate={embedded_delegate} />
    );
};

export default Embedded;
