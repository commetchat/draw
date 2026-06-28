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
import MatrixWidget from './pages/matrix';
import { Accessor, createSignal, Setter } from 'solid-js';
import { applySafeArea } from './utils/safe_area';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

const [saveProgress, setSaveProgress] = createSignal("");
export const useSaveProgress: () => [Accessor<string>, Setter<string>] = () => [saveProgress, setSaveProgress];

try {
  const urlParams = new URLSearchParams(window.location.search)
  console.log(urlParams);

  let safeArea = urlParams.get("safeArea");
  console.log(safeArea)

  if (safeArea != null && (safeArea.startsWith("$") == false)) {
    applySafeArea(safeArea);
  } else {
    console.log("Not applying safe area")
  }
} catch (_) {

}

render(() => <Router>
  <Route path="/" component={Root} />
  <Route path="/dev" component={MultiplayerTest} />
  <Route path="/dev-ui" component={DevUI} />
  <Route path="/embedded" component={Embedded} />
  <Route path="/matrix" component={MatrixWidget} />
</Router>, root!);
