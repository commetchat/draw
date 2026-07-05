import { createEffect, createSignal, Show, type Component } from 'solid-js';

import './ui.css';

import '@material/web/button/filled-button.js';
import '@material/web/iconbutton/filled-icon-button';
import '@material/web/iconbutton/filled-tonal-icon-button';
import '@material/web/fab/fab';
import '@material/web/icon/icon.js';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';
import '@material/web/dialog/dialog.js'
import '@material/web/progress/circular-progress.js';
import '@material/web/slider/slider.js';
import ColorPicker from '../molecules/color-picker/color-picker';
import { UIMessage } from '../bindings/ui_binding';
import { hsl2rgb, rgb2hex, rgb2hsl } from '../utils';
import h from 'solid-js/h';
import { useSaveProgress, useShowDialog } from '..';
import { applyTheme, argbFromHex, argbFromRgb, themeFromSourceColor } from '@material/material-color-utilities';
import { MdDialog } from '@material/web/dialog/dialog.js';


interface UIProps {
    callback: ((message: UIMessage) => void) | null;
    delegate: UIDelegate
}

interface UIDelegate {
    on_received_msg: ((msg: UIMessage) => void) | null
}

const UI: Component<UIProps> = (props) => {
    const colorPicker = "ColorPicker";
    const paintbrush = "Paintbrush";
    const lineArt = "LineArt";
    const showConsole = false;

    const [gameReady, setGameReady] = createSignal(false);
    const [paintColorHsl, setPaintColorHsl] = createSignal<[number, number, number]>([0, 1.0, 0.5])
    const [paintbrushWidth, setPaintbrushWidth] = createSignal(10.0);
    const [currentTool, setTool] = createSignal(lineArt);
    const [prevTool, setPrevTool] = createSignal<string | null>(null);
    const [fullscreen, setFullscreen] = createSignal(false);

    const [saveProgress, setSaveProgress] = useSaveProgress();
    const [showDialog, setShowDialog] = useShowDialog();

    const paintColorsRgb = () => {
        let hsl = paintColorHsl();
        return hsl2rgb(hsl[0], hsl[1], hsl[2])
    }

    let [getLines, setLines] = createSignal<string>("")

    if (showConsole) {
        let original = console.log;

        console.error = (e) => {
            let s = getLines();
            setLines(`${e}\n` + s.substring(0, 10000));
            original(e)
        }

        console.log = (e) => {

            let s = getLines();
            setLines(`${e}\n` + s.substring(0, 10000));
            original(e)
        }
    }

    function setCurrentTool(tool: string) {

        setPrevTool(currentTool());

        setTool(tool);
    }

    props.delegate.on_received_msg = (msg) => {

        if (msg.type == "SetColor") {
            let hsl = rgb2hsl(msg.r, msg.g, msg.b);

            setPaintColorHsl(hsl);

            let prev = prevTool();
            var tool = lineArt;

            if (prev != null) {
                tool = prev;
            }

            setCurrentTool(tool)

            postUiMessage({
                type: "SetTool",
                tool: tool as any,
                width: paintbrushWidth(),
                color: paintColorsRgb(),
            })
        }

        if (msg.type == "GameReady") {
            setGameReady(true);
        }
    }

    function postUiMessage(message: UIMessage) {

        if (props.callback != null) {
            props.callback!(message);
        }
    };

    createEffect(() => {

        if (currentTool() == lineArt) {
            postUiMessage({
                type: "SetTool",
                tool: lineArt,
                width: paintbrushWidth(),
                color: paintColorsRgb()
            })
        }

        if (currentTool() == paintbrush) {
            postUiMessage({
                type: "SetTool",
                tool: paintbrush,
                width: paintbrushWidth(),
                color: paintColorsRgb()
            })
        }

        if (currentTool() == colorPicker) {
            postUiMessage({
                type: "SetTool",
                tool: colorPicker
            })
        }
    });


    let dialog: any;

    createEffect(() => {
        console.log(dialog.children);
        let d = dialog.children[0] as MdDialog;

        if (showDialog()) {
            d.show();
        } else {
            d.close();
        }
    })

    function saveToBackend() {
        setShowDialog(true);
        postUiMessage({ type: "SaveToBackend" });
    }

    function toggleFullscreen() {
        var elem = document.documentElement;

        if (fullscreen()) {
            document.exitFullscreen();
            setFullscreen(false);
        } else {
            elem.requestFullscreen();
            setFullscreen(true);
        }
    }

    function setColor(hsl: number[]) {
        setPaintColorHsl([hsl[0], hsl[1], hsl[2]]);
    }



    return (

        <div class='pointer-events-none' style={"z-index: 2; position: absolute; top: 0; left: 0; width: 100%; height: 100%"}>


            <div class='pt-(--safe-area-top) pl-(--safe-area-left) pb-(--safe-area-bottom) pr-(--safe-area-right)'>

                <div ref={dialog}>
                    <md-dialog class="dialog m-auto">
                        <div slot="headline" class='text-(--md-sys-color-on-surface) ml-4 mt-4'> <h2>Saving...</h2></div>
                        <div slot="content" class='text-center flex items-center flex-col content-center justify-center'>
                            <md-circular-progress class='mt-6' indeterminate></md-circular-progress>

                            <div class='m-6'>
                                {saveProgress()}
                            </div>

                        </div>
                    </md-dialog>
                </div>

                <div class="pointer-events-auto flex gap-4 " style={"margin: 10px;"}>
                    <md-filled-button onclick={() => postUiMessage({ type: "LoadFile" })}> <div class='mx-4' >Open File</div></md-filled-button>
                    <md-filled-button onclick={() => postUiMessage({ type: "SaveFile" })}> <div class='mx-4' >Save File</div></md-filled-button>
                    <md-filled-button onclick={() => saveToBackend()}> <div class='mx-4' >{`Save`}</div></md-filled-button>
                </div>

                <Show when={gameReady() == false}>
                    <div class="pointer-events-auto absolute top-1/2 left-1/2">
                        <md-filled-button> <div class='mx-4' >Getting Ready...</div></md-filled-button>
                    </div>
                </Show>



                <div class="pointer-events-auto margin-auto">
                    {dialog}
                </div>



                <div class='pointer-events-auto absolute bottom-0 bg-blend-overlay pb-(--safe-area-bottom)' style={"filter: drop-shadow(0px 0px 1px gray);"} >
                    <div style={"margin: 10px; "}>
                        <md-slider max={500} oninput={(e) => setPaintbrushWidth((e.target as any).value)} value={paintbrushWidth()} ></md-slider>
                        <ColorPicker onchanged={setColor} hsl={paintColorHsl()}></ColorPicker>
                    </div>
                </div>

                <div class="tool-buttons ml-4 pointer-events-auto absolute top-1/3 bottom-1/2 flex flex-col gap-4">
                    <div>
                        <Show when={currentTool() != lineArt}>
                            <md-filled-tonal-icon-button onclick={() => setCurrentTool(lineArt)}>
                                <md-icon>stylus_fountain_pen</md-icon>
                            </md-filled-tonal-icon-button>
                        </Show>
                        <Show when={currentTool() == lineArt}>
                            <md-filled-icon-button onclick={() => setCurrentTool(lineArt)}>
                                <md-icon className='my-7'>stylus_fountain_pen</md-icon>
                            </md-filled-icon-button>
                        </Show>
                    </div>

                    <div>
                        <Show when={currentTool() != paintbrush}>
                            <md-filled-tonal-icon-button onclick={() => setCurrentTool(paintbrush)}>
                                <md-icon>brush</md-icon>
                            </md-filled-tonal-icon-button>
                        </Show>
                        <Show when={currentTool() == paintbrush}>
                            <md-filled-icon-button onclick={() => setCurrentTool(paintbrush)}>
                                <md-icon className='my-7'>brush</md-icon>
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
                </div>

                <div class="pointer-events-auto flex flex-col items-end justify-between gap-4 absolute bottom-0 pb-(--safe-area-bottom) pr-(--safe-area-right) right-0 m-4 ">
                    <md-fab variant="secondary" aria-label="Fullscreen" onclick={(e) => {
                        e.preventDefault();
                        toggleFullscreen();
                    }}>
                        <md-icon slot="icon">{fullscreen() ? "fullscreen_exit" : "fullscreen"}</md-icon>
                    </md-fab>

                    <md-fab variant="secondary" aria-label="Reset Camera" onclick={(e) => {
                        e.preventDefault();
                        return postUiMessage({ type: "ResetCamera" });
                    }}>
                        <md-icon slot="icon">cameraswitch</md-icon>
                    </md-fab>

                    <md-fab variant='tertiary' size="large" aria-label="Undo" onclick={(e) => {
                        e.preventDefault();
                        return postUiMessage({ type: "Undo" });
                    }}>
                        <md-icon slot="icon">undo</md-icon>
                    </md-fab>
                </div>

                <Show when={showConsole}>
                    <div class='text-white text-xs w-lvh h-svh'>
                        <textarea class='pointer-events-auto p-20 w-lvh h-1/3' disabled value={getLines()}>

                        </textarea>
                    </div>
                </Show>
            </div >
        </div >
    );
};


export default UI;
export type { UIDelegate };
