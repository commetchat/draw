export function downloadBlob(data: Uint8Array, fileName: string, mimeType: string) {

    var blob = new Blob([data], {
        type: mimeType
    });
    var url = window.URL.createObjectURL(blob);
    downloadURL(url, fileName);
    setTimeout(function () {
        return window.URL.revokeObjectURL(url);
    }, 1000);
};

export function downloadURL(data: string, fileName: string) {
    var a;
    a = document.createElement('a');
    a.href = data;
    a.download = fileName;
    document.body.appendChild(a);
    a.click();
    a.remove();
};