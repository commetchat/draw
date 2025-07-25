import { createSignal, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';
import App, { NetworkDelegate } from '../organisms/app';

let peerjs_delegate: NetworkDelegate = {
    send_to: function (message: Uint8Array, to: String): void {
        throw new Error('Function not implemented.');
    },

    on_received: null,
    on_peer_connected: null,
    on_peer_disconnected: null
}


const Root: Component = () => {
    return (
        <App instance_id='root' delegate={peerjs_delegate}></App>
    );
};

export default Root;
