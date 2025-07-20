import type { Component } from 'solid-js';

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
  await window.gameDatabase.init();
  game.default();
}

const App: Component = () => {

  initGame();

  return (
    <div class={styles.App}>

      <header>
        <div style={"width: 100vw; height: 100vh;  overflow: hidden;"}>
          <canvas style={"z-index: 1; position: relative;"} id="bevy-portal"></canvas>
        </div>
        <div style={"z-index: 2; position: absolute; top: 0; left: 0;"}>
          <md-filled-icon-button>test</md-filled-icon-button>
          <md-filled-button onclick={() => game.greet("Test")}>hello!</md-filled-button>
        </div>
      </header>
    </div >
  );
};

export default App;
