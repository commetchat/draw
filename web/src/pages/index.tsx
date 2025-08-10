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

        if (message.length > 64_000) {
            const blob = new Blob([new Uint8Array(message)], {
                type: "blob"
            });
            console.log("Sending blob");
            connection.send(blob);
        } else {
            connection.send(message);
        }

        console.log("Sending bytes to peer :" + message.length.toString());

    },

    on_received: null,
    on_peer_connected: null,
    on_peer_disconnected: null,
    on_ready: null,
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
            let connection = peer!.connect(hostId);

            console.log("Created connection!")
            connection.on("open", () => onConnectionOpened(connection))
        }

        onReady(id);
    });

    peer.on("connection", (conn) => {
        console.log("Connection created!")
        console.log(conn)
        conn.on("open", () => onConnectionOpened(conn))
    })

    peer.on("error", (err) => {
        console.log("Peer error");
        console.log(err);
    })
}

function onConnectionOpened(conn: DataConnection) {
    console.log("Connection opened!")
    connections.set(conn.peer, conn);

    peerjs_delegate.on_peer_connected!(conn.peer)

    conn.on("data", (data) => {
        let bytes = new Uint8Array(data as any);
        peerjs_delegate.on_received!(bytes, conn.peer)
    });

    conn.on("error", (err) => {
        console.log("Connection Error");
        console.log(err);
    });
}

function onReady(id: string) {
    peerjs_delegate.on_ready!(id);
}

const Root: Component = () => {
    return (
        <App instance_id='root' network_delegate={peerjs_delegate} game_delegate={game_delegate}></App>
    );
};

export default Root;
