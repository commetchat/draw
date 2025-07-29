import { createEffect, createSignal, Show, type Component } from 'solid-js';

import './ui.css';

import '@material/web/button/filled-button.js';
import '@material/web/iconbutton/filled-icon-button';
import '@material/web/iconbutton/filled-tonal-icon-button';
import '@material/web/fab/fab';
import '@material/web/icon/icon.js';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';

import '@material/web/slider/slider.js';
import ColorPicker from '../molecules/color-picker/color-picker';
import { UIMessage } from '../bindings/ui_binding';
import { hsl2rgb, rgb2hex, rgb2hsl } from '../utils';
import h from 'solid-js/h';

interface UIProps {
    callback: ((message: UIMessage) => void) | null;
    delegate: UIDelegate
}

interface UIDelegate {
    on_received_msg: ((msg: UIMessage) => void) | null
}

const UI: Component<UIProps> = (props) => {
    const [gameReady, setGameReady] = createSignal(false);
    const [paintColorHsl, setPaintColorHsl] = createSignal<[number, number, number]>([0, 1.0, 0.5])
    const [paintbrushWidth, setPaintbrushWidth] = createSignal(10.0);
    const [currentTool, setCurrentTool] = createSignal("Paintbrush");

    console.log("Test!");
    const colorPicker = "ColorPicker";
    const paintbrush = "Paintbrush";

    const paintColorsRgb = () => {
        let hsl = paintColorHsl();
        return hsl2rgb(hsl[0], hsl[1], hsl[2])
    }

    props.delegate.on_received_msg = (msg) => {
        console.log("Received message:");
        console.log(msg);

        if (msg.type == "SetColor") {
            let hsl = rgb2hsl(msg.r, msg.g, msg.b);

            setPaintColorHsl(hsl);

            setCurrentTool(paintbrush);
            console.log("Set colors!");

            postUiMessage({
                type: "SetTool",
                tool: paintbrush,
                width: paintbrushWidth(),
                color: paintColorsRgb()
            })
        }

        if (msg.type == "GameReady") {
            setGameReady(true);
        }

    }

    function postUiMessage(message: UIMessage) {
        console.log(message);
        if (props.callback != null) {
            props.callback!(message);
        }
    };

    createEffect(() => {
        console.log("Sending ui message!");

        if (currentTool() == paintbrush) {
            postUiMessage({
                type: "SetTool",
                tool: paintbrush,
                width: paintbrushWidth(),
                color: paintColorsRgb()
            })

            console.log(paintColorHsl());
        }

        if (currentTool() == colorPicker) {
            postUiMessage({
                type: "SetTool",
                tool: colorPicker
            })
        }
    });

    return (
        <div style={`--md-sys-color-primary: ${rgb2hex(hsl2rgb(paintColorHsl()[0], 0.2, 0.5))}; --md-sys-color-secondary-container: ${rgb2hex(hsl2rgb(paintColorHsl()[0], 0.2, 0.8))}`}>

            <div class='pointer-events-none' style={"z-index: 2; position: absolute; top: 0; left: 0; width: 100%; height: 100%"}>

                <div class="pointer-events-auto flex justify-between gap-2 " style={"margin: 10px; position: absolute; left: 0;"}>
                    <md-filled-button onclick={() => postUiMessage({ type: "LoadFile" })}> <div class='mx-4' >Open File</div></md-filled-button>
                    <md-filled-button onclick={() => postUiMessage({ type: "SaveFile" })}> <div class='mx-4' >Save File</div></md-filled-button>
                </div>

                <Show when={gameReady() == false}>
                    <div class="pointer-events-auto absolute top-1/2 left-1/2">
                        <md-filled-button> <div class='mx-4' >Getting Ready...</div></md-filled-button>
                    </div>
                </Show>


                <div class='pointer-events-auto absolute bottom-0 bg-blend-overlay' style={"filter: drop-shadow(0px 0px 1px gray);"} >
                    <div style={"margin: 10px; "}>
                        <md-slider max={500} oninput={(e) => setPaintbrushWidth((e.target as any).value)} value={paintbrushWidth()} ></md-slider>
                        <ColorPicker onchanged={setPaintColorHsl} hsl={paintColorHsl()}></ColorPicker>
                    </div>
                </div>

                <div class="tool-buttons ml-4 pointer-events-auto absolute top-1/3 bottom-1/2 flex flex-col gap-2">
                    <div>
                        <Show when={currentTool() != paintbrush}>
                            <md-filled-tonal-icon-button onclick={() => setCurrentTool(paintbrush)}>
                                <md-icon>stylus</md-icon>
                            </md-filled-tonal-icon-button>
                        </Show>
                        <Show when={currentTool() == paintbrush}>
                            <md-filled-icon-button onclick={() => setCurrentTool(paintbrush)}>
                                <md-icon className='my-7'>stylus</md-icon>
                            </md-filled-icon-button>
                        </Show>
                    </div>
                    <div>
                        <Show when={currentTool() != colorPicker}>
                            <md-filled-tonal-icon-button onclick={() => setCurrentTool(colorPicker)}>
                                <md-icon>dropper_eye</md-icon>
                            </md-filled-tonal-icon-button>
                        </Show>
                        <Show when={currentTool() == colorPicker}>
                            <md-filled-icon-button onclick={() => setCurrentTool(colorPicker)}>
                                <md-icon>dropper_eye</md-icon>
                            </md-filled-icon-button>
                        </Show>
                    </div>
                    <div>
                        <md-filled-tonal-icon-button>
                            <md-icon>comic_bubble</md-icon>
                        </md-filled-tonal-icon-button>
                    </div>
                </div>

                <div class="pointer-events-auto flex justify-between gap-2 absolute bottom-0 right-0 m-4 ">
                    <md-fab aria-label="Edit" onclick={() => postUiMessage({ type: "Undo" })}>
                        <md-icon slot="icon">undo</md-icon>
                    </md-fab>
                </div>

            </div >
        </div>

    );
};


export default UI;
export type { UIDelegate };
