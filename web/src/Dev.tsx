import { createSignal, For, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import * as game from './bevy/draw-bevy';

import '@material/web/button/filled-button.js';

import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';
import { WebDatabase } from './web_database';


declare global {
    interface Window { gameDatabase: WebDatabase; }
}

const MultiplayerTest: Component = () => {


    let [getLines, setLines] = createSignal<string>("")
    let original = console.log;

    let instances = ['instance0', 'instance1'];


    window.onmessage = (message) => {
        console.log("Received message");
        console.log(message);
    }

    const connect = () => {
        instances.forEach((sender) => {
            instances.forEach((receiver) => {
                if (sender == receiver) return;

                let receiver_frame = document.getElementById(receiver) as any
                let msg = {
                    "type": "peerconnect",
                    "info": JSON.stringify({ "from": sender }),
                    "body": null
                }

                console.log(`${sender} connecting to ${receiver}`);
                console.log(receiver_frame)

                receiver_frame.contentWindow.postMessage(msg);
            })
        });
    }

    return (
        <div class={styles.App}>
            <md-filled-button onclick={connect}>Connect</md-filled-button>
            <br></br>
            <For each={instances}>
                {
                    (item) => <iframe id={item} width="1280" height="720" src={`/embedded?id=${item}`} />
                }
            </For>
        </div >
    );
};

export default MultiplayerTest;
