function camelToKebab(str: string) {
    return str.replace(/[A-Z]/g, m => `-${m.toLowerCase()}`);
}

export function applyMaterialTheme(scheme: any) {
    const root = document.documentElement;

    for (const [key, value] of Object.entries(scheme)) {
        if (key === "brightness") continue;

        let cssKey = `--md-sys-color-${camelToKebab(key)}`;

        console.log("Setting: ", cssKey)
        root.style.setProperty(
            cssKey,
            value as any
        );
    }
}
