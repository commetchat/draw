export function merge_arrays_uint8(a: Uint8Array, b: Uint8Array): Uint8Array {
    var mergedArray = new Uint8Array(a.length + b.length);
    mergedArray.set(a);
    mergedArray.set(b, a.length);

    return mergedArray;
}

export function merge_arrays_uint32(a: Uint32Array, b: Uint32Array): Uint32Array {
    var mergedArray = new Uint32Array(a.length + b.length);
    mergedArray.set(a);
    mergedArray.set(b, a.length);

    return mergedArray;
}
