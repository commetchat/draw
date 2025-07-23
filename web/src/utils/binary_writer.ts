
export class BinaryWriter {
    private buffer: ArrayBuffer;
    private view: DataView;
    private offset: number;
    private littleEndian: boolean;

    constructor(size: number = 1024) {
        this.buffer = new ArrayBuffer(size);
        this.view = new DataView(this.buffer);
        this.offset = 0;
        this.littleEndian = true;
    }

    ensureCapacity(bytes: number) {
        if (this.offset + bytes > this.buffer.byteLength) {
            const newBuffer = new ArrayBuffer(this.buffer.byteLength * 2 + bytes);
            new Uint8Array(newBuffer).set(new Uint8Array(this.buffer));
            this.buffer = newBuffer;
            this.view = new DataView(this.buffer);
        }
    }

    writeUint32(value: number) {
        this.ensureCapacity(4);
        this.view.setUint32(this.offset, value, this.littleEndian);
        this.offset += 4;
    }

    writeInt32(value: number) {
        this.ensureCapacity(4);
        this.view.setInt32(this.offset, value, this.littleEndian);
        this.offset += 4;
    }

    writeUint16(value: number) {
        this.ensureCapacity(2);
        this.view.setUint16(this.offset, value, this.littleEndian);
        this.offset += 2;
    }

    writeUint8(value: number) {
        this.ensureCapacity(1);
        this.view.setUint8(this.offset, value);
        this.offset += 1;
    }

    writeFloat32(value: number) {
        this.ensureCapacity(4);
        this.view.setFloat32(this.offset, value, this.littleEndian);
        this.offset += 4;
    }

    writeFloat64(value: number) {
        this.ensureCapacity(8);
        this.view.setFloat64(this.offset, value, this.littleEndian);
        this.offset += 8;
    }

    writeBytes(bytes: Uint8Array) {
        this.ensureCapacity(bytes.length);
        new Uint8Array(this.buffer, this.offset, bytes.length).set(bytes);
        this.offset += bytes.length;
    }


    writeString(text: string) {
        var utf8Encode = new TextEncoder();
        var bytes = utf8Encode.encode(text);
        this.writeUint32(bytes.length);
        this.writeBytes(bytes);
    }

    getBuffer(): ArrayBuffer {
        return this.buffer.slice(0, this.offset);
    }
}
