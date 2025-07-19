import type { Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import * as game from './bevy/draw-bevy';


const App: Component = () => {

  game.default();

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <canvas id="bevy-portal" tabindex="-1" data-raw-handle="1" ></canvas>
      </header>
    </div >
  );
};

export default App;
