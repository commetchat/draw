
export class BinaryReader {
    private view: DataView;
    private offset: number;
    private littleEndian: boolean;

    constructor(private buffer: ArrayBuffer) {
        this.view = new DataView(buffer);
        this.offset = 0;
        this.littleEndian = true;
    }

    readUint32(): number {
        const value = this.view.getUint32(this.offset, this.littleEndian);
        this.offset += 4;
        return value;
    }

    readInt32(): number {
        const value = this.view.getInt32(this.offset, this.littleEndian);
        this.offset += 4;
        return value;
    }

    readUint16(): number {
        const value = this.view.getUint16(this.offset, this.littleEndian);
        this.offset += 2;
        return value;
    }

    readUint8(): number {
        const value = this.view.getUint8(this.offset);
        this.offset += 1;
        return value;
    }

    readFloat32(): number {
        const value = this.view.getFloat32(this.offset, this.littleEndian);
        this.offset += 4;
        return value;
    }

    readFloat64(): number {
        const value = this.view.getFloat64(this.offset, this.littleEndian);
        this.offset += 8;
        return value;
    }

    readBytes(length: number): Uint8Array {
        const bytes = new Uint8Array(this.buffer, this.offset, length);
        this.offset += length;
        return bytes;
    }

    readString(): string {
        let length = this.readUint32();
        let bytes = this.readBytes(length);
        let decoder = new TextDecoder();
        let result = decoder.decode(bytes);
        return result;
    }

    getPosition(): number {
        return this.offset;
    }

    setPosition(pos: number): void {
        this.offset = pos;
    }
}
