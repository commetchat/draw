import { createSignal, type Component } from 'solid-js';

import './color-picker.css';

import '@material/web/button/filled-button.js';

import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';

import '@material/web/slider/slider.js';

// input: h as an angle in [0,360] and s,l in [0,1] - output: r,g,b in [0,1]
function hsl2rgb(h: number, s: number, l: number) {
    let a = s * Math.min(l, 1 - l);
    let f = (n: number, k = (n + h / 30) % 12) => l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1);
    return [f(0), f(8), f(4)];
}

function rgb2hex(numbers: number[]): string {

    var hex = numbers.map((n) => {
        return Math.round(255 * n)
            .toString(16)
            .padStart(2, "0");
    });
    let result = `#${hex[0]}${hex[1]}${hex[2]}`

    return result;
}

const ColorPicker: Component = () => {
    const [color, setColor] = createSignal("#aabbcc")

    const [h, setHue] = createSignal(360)
    const [s, setSaturation] = createSignal(0.5);
    const [l, setLightness] = createSignal(0.5);



    const hueColor = () => rgb2hex(hsl2rgb(h(), 0.5, 0.5));
    const satColor = () => rgb2hex(hsl2rgb(h(), s(), 0.5));
    const finalColor = () => rgb2hex(hsl2rgb(h(), s(), l()));

    return (
        <div class='flex w-[500px] items-center justify-center' style={`--md-sys-color-primary: ${finalColor()};`}>
            <div>

                <div class='rounded-full w-20 h-20' style={`background-color: ${finalColor()};`} >
                </div>

            </div>
            <div class='flex-1'>
                <div class='hue-picker'>

                    <md-slider value={h()} max={360} min={0} oninput={(v) => setHue((v as any).target.value)} onchange={(v) => setHue((v as any).target.value)}></md-slider>
                </div>
                <div class='hue-picker ml-2' style={`--md-slider-inactive-track-color: linear-gradient(90deg, hsl(${h()}, 0%, ${l() * 100}%), hsl(${h()}, 50%, ${l() * 100}%), hsl(${h()}, 100%, ${l() * 100}%));`}>
                    <md-slider value={s()} min={0} max={1.0} step={0.01} oninput={(v) => setSaturation((v as any).target.value)} onchange={(v) => setSaturation((v as any).target.value)} ></md-slider>
                </div>
                <div class='hue-picker' style={`--md-slider-inactive-track-color: linear-gradient(90deg, hsl(${h()}, ${s() * 100}%, 0%), hsl(${h()}, ${s() * 100}%, 50%), hsl(${h()}, ${s() * 100}%, 100%));`}>
                    <md-slider value={l()} ticks min={0} max={1.0} step={0.1} oninput={(v) => setLightness((v as any).target.value)} onchange={(v) => setLightness((v as any).target.value)}  ></md-slider>
                </div>
            </div>
        </div >


    );
};


export default ColorPicker;
