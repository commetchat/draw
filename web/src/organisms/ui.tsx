import { createSignal, type Component } from 'solid-js';

import './ui.css';

import '@material/web/button/filled-button.js';

import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';

import '@material/web/slider/slider.js';
import ColorPicker from '../molecules/color-picker/color-picker';
import { UIMessage } from '../bindings/ui_binding';

interface UIProps {
    callback: ((message: UIMessage) => void) | null;
}


const UI: Component<UIProps> = (props) => {
    const [color, setColor] = createSignal("#aabbcc")

    function postUiMessage(message: UIMessage) {
        if (props.callback != null) {
            props.callback!(message);
        }
    };

    return (
        <div class='pointer-events-none' style={"z-index: 2; position: absolute; top: 0; left: 0; width: 100%; height: 100%"}>

            <div class="pointer-events-auto flex justify-between gap-2 " style={"margin: 10px; position: absolute; left: 0;"}>
                <md-filled-button onclick={() => postUiMessage({ type: "LoadFile" })}> <div class='mx-4' >Open File</div></md-filled-button>
                <md-filled-button onclick={() => postUiMessage({ type: "SaveFile" })}> <div class='mx-4' >Save File</div></md-filled-button>
            </div>

            <div class='pointer-events-auto absolute bottom-0 bg-blend-overlay' style={"filter: drop-shadow(0px 0px 1px gray);"} >
                <div style={"margin: 10px; "}>
                    <md-slider onchange={() => postUiMessage({ type: "SetTool", tool: "Paintbrush", width: 10, color: [0, 1, 0] })} ></md-slider>
                    <ColorPicker></ColorPicker>
                </div>
            </div>
        </div >

    );
};


export default UI;
