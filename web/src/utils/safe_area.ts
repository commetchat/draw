
export function applySafeArea(safeArea: string) {
    console.log("Applying safe area!");
    console.log(safeArea);
    var parts = safeArea.split(",");


    console.log(parts)
    const root = document.documentElement;

    root.style.setProperty(`--safe-area-left`, parts[0]);
    root.style.setProperty(`--safe-area-top`, parts[1]);
    root.style.setProperty(`--safe-area-right`, parts[2]);
    root.style.setProperty(`--safe-area-bottom`, parts[3]);

}