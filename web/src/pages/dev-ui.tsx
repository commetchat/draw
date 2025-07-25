import { createSignal, For, type Component } from 'solid-js';



import UI from '../organisms/ui';



const DevUI: Component = () => {

    return (
        <div class='w-lvw h-lvh bg-neutral-900'>
            <UI callback={null}></UI>
        </div>
    );
};

export default DevUI;
