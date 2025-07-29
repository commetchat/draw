import { createEffect, createSignal, type Component } from 'solid-js';

import './color-picker.css';

import '@material/web/button/filled-button.js';

import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';

import '@material/web/slider/slider.js';
import { hsl2rgb, rgb2hex } from '../../utils';



interface ColorPickerProps {
    onchanged: ((hsl: number[]) => void) | null;
    hsl: number[],
}


const ColorPicker: Component<ColorPickerProps> = (props) => {
    const [color, setColor] = createSignal("#aabbcc")

    const h = (): number => {
        return props.hsl[0];
    }

    const s = (): number => {
        return props.hsl[1];
    }

    const l = (): number => {
        return props.hsl[2];
    }

    const hueColor = () => rgb2hex(hsl2rgb(h(), 0.5, 0.5));
    const satColor = () => rgb2hex(hsl2rgb(h(), s(), 0.5));
    const finalColor = () => rgb2hex(hsl2rgb(h(), s(), l()));

    var set_h = h();
    var set_s = s();
    var set_l = l();

    const hueEvent = (e: Event) => {
        set_h = (e as any).target.value;
        reportChange()
    };

    const satEvent = (e: Event) => {
        set_s = (e as any).target.value
        reportChange()
    };

    const lgtEvent = (e: Event) => {
        set_l = (e as any).target.value;
        reportChange()
    };

    createEffect(() => {
        set_h = props.hsl[0];
        set_s = props.hsl[1];
        set_l = props.hsl[2];
    })

    const reportChange = () => {
        console.log(set_h, set_s, set_l);
        console.log("Setting color: ", color);
        if (props.onchanged != null) {
            props.onchanged([set_h, set_s, set_l]);
        }
    }

    return (
        <div class='flex w-[500px] items-center justify-center' style={`--md-sys-color-primary: ${finalColor()};`}>
            <div>

                <div class='rounded-full w-20 h-20' style={`background-color: ${finalColor()};`} >
                </div>

            </div>
            <div class='flex-1'>
                <div class='hue-picker'>

                    <md-slider value={h()} max={360} min={0} oninput={hueEvent} onchange={(v) => { hueEvent(v); reportChange() }}></md-slider>
                </div>
                <div class='hue-picker ml-2' style={`--md-slider-inactive-track-color: linear-gradient(90deg, hsl(${h()}, 0%, ${l() * 100}%), hsl(${h()}, 50%, ${l() * 100}%), hsl(${h()}, 100%, ${l() * 100}%));`}>
                    <md-slider value={s()} min={0} max={1.0} step={0.01} oninput={satEvent} onchange={(v) => { satEvent(v); reportChange() }} ></md-slider>
                </div>
                <div class='hue-picker' style={`--md-slider-inactive-track-color: linear-gradient(90deg, hsl(${h()}, ${s() * 100}%, 0%), hsl(${h()}, ${s() * 100}%, 50%), hsl(${h()}, ${s() * 100}%, 100%));`}>
                    <md-slider value={l()} ticks min={0} max={1.0} step={0.1} oninput={lgtEvent} onchange={(v) => { lgtEvent(v); reportChange() }}  ></md-slider>
                </div>
            </div>
        </div >


    );
};


export default ColorPicker;
