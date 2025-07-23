/* @refresh reload */
import { render } from 'solid-js/web';

import './index.css';
import App from './App';
import { Route, Router } from '@solidjs/router';
import MultiplayerTest from './Dev';
import Root from './Root';
import Embedded from './Embedded';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

render(() => <Router>
  <Route path="/" component={Root} />
  <Route path="/dev" component={MultiplayerTest} />
  <Route path="/embedded" component={Embedded} />
</Router>, root!);
