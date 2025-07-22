import { createSignal, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import * as game from './bevy/draw-bevy';


import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';
import { WebDatabase } from './web_database';


declare global {
  interface Window { gameDatabase: WebDatabase; }
}

async function initGame() {
  window.gameDatabase = new WebDatabase()
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
    reader.readAsText(file, 'UTF-8');

    // here we tell the reader what to do when it's done reading...
    reader.onload = readerEvent => {
      var content = (readerEvent as any).target.result; // this is the content!
      console.log(content);
    }
  }

  input.click();
}

const App: Component = () => {

  initGame();

  let [getLines, setLines] = createSignal<string>("")
  let original = console.log;

  // console.log = (e) => {
  //   original(e);
  // 
  //   let s = getLines();
  //   setLines(`${e}\n` + s);
  // }



  return (
    <div class={styles.App}>

      <header>
        <div style={"width: 100vw; height: 100vh;  overflow: hidden;"}>
          <canvas style={"z-index: 1; position: relative;"} id="bevy-portal"></canvas>
        </div>
        <div style={"z-index: 2; position: absolute; top: 0; left: 0;"}>
          <md-filled-icon-button onclick={openFile} >Open File</md-filled-icon-button>
          <md-filled-button onclick={() => game.greet("Test")}>hello!</md-filled-button>
          <textarea style={"width: 75vw; height: 20vh; position: absolute"} disabled value={getLines()}>
          </textarea>

        </div>
      </header>
    </div >
  );
};

export default App;
