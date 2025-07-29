import { createSignal, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';
import App, { GameDelegate, NetworkDelegate } from '../organisms/app';
import Peer, { DataConnection } from 'peerjs';

let connections = new Map()

let peerjs_delegate: NetworkDelegate = {
    send_to: function (message: Uint8Array, to: String): void {
        let connection = connections.get(to) as DataConnection;
        connection.send(message);
    },

    on_received: null,
    on_peer_connected: null,
    on_peer_disconnected: null
}

let game_delegate: GameDelegate = {
    on_ready: function (): void {
        console.log("Game is ready!");
        startConnection();
    }
}


function startConnection() {
    console.log("Starting connection!")

    let id = localStorage.getItem("userid")
    var peer: Peer | null = null;

    if (id == null) {
        peer = new Peer()
    } else {
        peer = new Peer(id)
    }

    peer.on('open', function (id) {
        console.log('peer: My peer ID is: ' + id);
        localStorage.setItem("userid", id)

        const urlParams = new URLSearchParams(window.location.search);
        const hostId = urlParams.get('room');

        if (hostId != null) {
            let connection = peer!.connect(hostId)
            console.log("Created connection!")
            connection.on("open", () => onConnectionOpened(connection))
        }
    });

    peer.on("connection", (conn) => {
        console.log("Connection created!")
        console.log(conn)
        conn.on("open", () => onConnectionOpened(conn))
    })
}

function onConnectionOpened(conn: DataConnection) {
    console.log("Connection opened!")
    connections.set(conn.peer, conn);

    peerjs_delegate.on_peer_connected!(conn.peer)

    conn.on("data", (data) => {
        peerjs_delegate.on_received!(new Uint8Array(data as any), conn.peer)
    });
}

const Root: Component = () => {
    return (
        <App instance_id='root' network_delegate={peerjs_delegate} game_delegate={game_delegate}></App>
    );
};

export default Root;
