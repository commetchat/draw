/* @refresh reload */
import { render } from 'solid-js/web';

import './index.css';
import App from './organisms/app';
import { Route, Router } from '@solidjs/router';
import Root from './pages';
import MultiplayerTest from './pages/dev';
import Embedded from './pages/embedded';
import UI from './organisms/ui';
import DevUI from './pages/dev-ui';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

render(() => <Router>
  <Route path="/" component={Root} />
  <Route path="/dev" component={MultiplayerTest} />
  <Route path="/dev-ui" component={DevUI} />
  <Route path="/embedded" component={Embedded} />
</Router>, root!);
