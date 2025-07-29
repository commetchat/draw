import { createSignal, For, type Component } from 'solid-js';



import UI, { UIDelegate } from '../organisms/ui';



const DevUI: Component = () => {
    let ui_delegate: UIDelegate = {
        on_received_msg: null
    };
    return (
        <div class='w-lvw h-lvh bg-neutral-900'>
            <UI callback={null} delegate={ui_delegate}></UI>
        </div>
    );
};

export default DevUI;
