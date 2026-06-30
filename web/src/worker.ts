import * as game from './bevy/draw-bevy';
import { AsyncTaskQueue } from './utils/async_task_queue';
import { BinaryWriter } from './utils/binary_writer';
import { downloadBlob } from './utils/download';
import { merge_arrays_uint32, merge_arrays_uint8 } from './utils/merge_arrays';

let db: IDBDatabase | null

console.error = (message, parms) => postAlert(`ERROR: ${message}`)

self.onerror = function(message) {
  postAlert(`ERROR: ${message}`)
};

self.onmessage = function (e) {
    if (e.data.type == "init_db") {
        let id = e.data.data;

        initDb(id);
    }

    if (e.data.type == "store_multiple_strokes") {
        store_multiple_strokes(e.data.data);
    }

    if (e.data.type == "load_mesh_for_chunk") {
        load_mesh_for_chunk(e.data.data);
    }

    if (e.data.type == "append_chunk_data") {
        append_chunk_data(e.data.data);
    }

    if (e.data.type == "save_to_file") {
        save_to_file();
    }

    if (e.data.type == "save_to_backend") {
        save_to_backend();
    }

    if (e.data.type == "send_strokes_to_user") {
        send_strokes_to_user(e.data.data);
    }

    if (e.data.type == "delete_stroke") {
        console.log(e.data);
        delete_stroke(e.data.data);
    }
}

const strokes = "stroke"
const mesh = "mesh"

type MeshData = {
    chunk_key: string,
    vertices: Uint8Array,
    indices: Uint32Array,
    colors: Uint8Array,
}

function clearDb(instance_id: string): Promise<void> {
    console.log("Clearing database");
    return new Promise(resolve => {
        var request = indexedDB.deleteDatabase(`${instance_id}_strokeStorage`);
        console.log("Created request");


        resolve();

    });
}

async function initDb(instance_id: string) {
    console.log("Initializing database");
    await clearDb(instance_id);
    await new Promise((resolve, reject) => {

        console.log("Creating database");
        let id = `${instance_id}_strokeStorage`;
        console.log(id);
        var request = indexedDB.open(id, 3);

        request.onupgradeneeded = event => {
            console.log("Upgrade needed");
            var db = (event.target as IDBOpenDBRequest).result;

            var objectStore = db.createObjectStore(strokes, {
                keyPath: "id"
            })

            objectStore.createIndex("chunk_key", "chunk_key", { unique: false });
            objectStore.createIndex("id_random", "id_random", { unique: false });
            objectStore.createIndex("owner_id", "owner_id", { unique: false });
            objectStore.createIndex("timestamp", "timestamp", { unique: false });

            var mesh_store = db.createObjectStore(mesh, {
                keyPath: "chunk_key"
            });
        }

        request.onsuccess = event => {
            console.log("Opened database!");
            db = (event.target as IDBOpenDBRequest).result;
            self.postMessage({ type: "db_init" });
            resolve(null);
        }

        request.onerror = event => {
            console.log("Failed to open database!");
            resolve(null);
        }

        request.onblocked = event => {
            console.log("Database blocked!");
            resolve(null);
        }
    });
    console.log("Done!");
}


function store_multiple_strokes(items: [game.StrokeData]) {
    const tx = db?.transaction([strokes, mesh], "readwrite");


    let meshes_to_append: game.StrokeData[] = [];

    tx!.oncomplete = () => {
        postMessage({
            type: "append_mesh_data",
            data: meshes_to_append,
        })
    }

    tx!.onerror = (err) => {
        postAlert(`Transaction error when saving strokes: ${err}`,);
        console.log("Transaction error: ", err);
    }

    const store = tx?.objectStore(strokes);

    let keys = new Set<String>()

    items.forEach((e) => {
        keys.add(e.chunk_key);
    });

    let chunk_keys = keys.keys().toArray().sort();

    let meshes = tx?.objectStore(mesh);
    let mesh_data_request = meshes!.getAll(IDBKeyRange.bound(chunk_keys[0], chunk_keys[chunk_keys.length - 1]));

    mesh_data_request.onsuccess = (ev) => {
        let result = (ev.target as IDBRequest).result as MeshData[];

        let mesh_map: Map<string, MeshData> = new Map();

        result.forEach((mesh) => {
            mesh_map.set(mesh.chunk_key, mesh);
        });


        items.forEach((data) => {
            console.log("Storing stroke by user: ", data.owner_id)
            let mesh = mesh_map.get(data.chunk_key);

            let vertex_offset = 0;

            let merged_indices = data.index_data!;

            if (mesh == null) {
                mesh = {
                    chunk_key: data.chunk_key,
                    indices: data.index_data!,
                    colors: data.color_data!,
                    vertices: data.vertex_data!,
                };
            } else {

                vertex_offset = mesh.vertices.length / (3 * 4);
                let arr = Uint32Array.from(data.index_data!);

                for (let i = 0; i < arr.length; i++) {
                    arr[i] = arr[i] + vertex_offset;
                }

                merged_indices = arr;

                mesh = {
                    chunk_key: data.chunk_key,
                    indices: merge_arrays_uint32(mesh.indices, arr),
                    vertices: merge_arrays_uint8(mesh.vertices, data.vertex_data!),
                    colors: merge_arrays_uint8(mesh.colors, data.color_data!),
                }
            }

            console.log("Inserting stroke with id: ", data.id);
            console.log(data);

            let result = {
                id: data.id,
                id_random: data.id_random,
                chunk_key: data.chunk_key,
                owner_id: data.owner_id,
                timestamp: data.timestamp,
                origin_x: data.origin_x,
                vertex_offset: vertex_offset,
                num_verts: data.num_verts,
                origin_y: data.origin_y,
                stroke_data: data.stroke_data,
                source: data.source,
            }


            store?.put(result);

            let return_result = {
                vertex_data: data.vertex_data,
                index_data: merged_indices,
                color_data: data.color_data,
                ...result
            }

            return_result.stroke_data = new Uint8Array();
            meshes_to_append.push(return_result as game.StrokeData);


            mesh_map.set(data.chunk_key, mesh);
        })

        mesh_map.values().forEach((e) => meshes!.put(e));

        tx?.commit();
    }

    mesh_data_request.onerror = (ev) => {
        postAlert(`Transaction error when saving strokes: ${ev}`,);
        console.log("Failed to get mesh from db")
    }
}

function load_mesh_for_chunk(id: string) {
    const tx = db?.transaction(mesh, "readonly");
    const store = tx?.objectStore(mesh);
    var request = store?.get(IDBKeyRange.only(id))


    request!.onsuccess = (event) => {
        let result = (event.target as IDBRequest).result;

        if (result != null) {
            let m = result as MeshData;

            postMessage({
                type: "loaded_mesh_for_chunk",
                data: {
                    chunk_key: id,
                    vertex_data: m.vertices.buffer,
                    index_data: m.indices.buffer,
                    color_data: m.colors.buffer,
                }
            }, {
                transfer: [m.vertices.buffer, m.indices.buffer, m.colors.buffer]
            });

        }
    }

}

function send_strokes_to_user(userid: string) {
    const tx = db?.transaction(strokes, "readonly");
    const store = tx?.objectStore(strokes);

    var count = store!.count()
    count.onsuccess = function () {
        let num_strokes = count.result;
        send_stroke_index_to_user(userid, 0, num_strokes)
    }
}

function send_stroke_index_to_user(userid: string, index: number, count: number) {
    const tx = db?.transaction(strokes, "readonly");
    const store = tx?.objectStore(strokes);



    var cursorRequest = store!.index('id_random').openCursor(null, 'next');
    let has_advanced = false;

    cursorRequest.onsuccess = function (e) {

        var cursor = (e as any).target.result;

        if (cursor == null) {
            console.log("Done sending everything!");
            return;
        }

        if (index != 0) {
            if (has_advanced == false) {
                cursor.advance(index);
                has_advanced = true;
                return;
            }
        }

        if (index % 50 == 0) {
            console.log(`Sending Strokes... ${index} / ${count}  ${Math.round((index / count) * 10000) / 100}%`,);
        }

        let stroke = cursor.value as game.StrokeData;

        var writer = new BinaryWriter();
        writer.writeUint16(1);
        writeStroke(writer, stroke);

        let bytes = writer.getBuffer();

        postMessage({
            type: "send_to_user",
            data: {
                user: userid,
                data: bytes,
            }
        }, {
            transfer: [bytes]
        })

        setTimeout(() => {
            send_stroke_index_to_user(userid, index + 1, count);
        }, 50)
    };

}

async function save_to_backend() {
    let keys = await getChunkKeys();

    console.log("Received all keys: ", keys);

    let array = keys.values().toArray();

    for (var i = 0; i < array.length; i++) {
        var chunk = array[i];

        console.log("Saving chunk: ", chunk);

        var data = await writeChunk(chunk);
        console.log("Received data: ", data);

        postMessage({
            type: "save_chunk",
            data: {
                chunk_id: chunk,
                data: data,
            }
        }, {
            transfer: [data]
        })
    }
}

function postAlert(text: string) {
    postMessage({
        type: "alert",
        data: text
    })
}

function getChunkKeys(): Promise<Set<string>> {

    const p = new Promise<Set<string>>((resolve, reject) => {

        const tx = db?.transaction(strokes, "readonly");
        const store = tx?.objectStore(strokes);

        var cursorRequest = store!.index('chunk_key').openCursor(null, 'next');

        let keys = new Set<string>();

        cursorRequest.onsuccess = function (e) {

            var cursor = (e as any).target.result;
            if (cursor) {
                let stroke = cursor.value as game.StrokeData;
                if (stroke.source == game.StrokeSource.User) {
                    if (keys.has(stroke.chunk_key) == false) {
                        console.log("Adding key: ", stroke.chunk_key);
                        keys.add(stroke.chunk_key);
                    }
                }

                cursor.continue();
            } else {


                console.log("Found all keys: ");
                console.log(keys);
                let values = keys.values().toArray();

                resolve(keys);
            }
        }
    });

    return p;
}

function writeChunk(chunk_id: string): Promise<ArrayBuffer> {

    const p = new Promise<ArrayBuffer>((resolve, reject) => {

        const tx = db?.transaction(strokes, "readwrite");
        const store = tx?.objectStore(strokes);

        var writer = new BinaryWriter();
        let magic = new TextEncoder().encode("draw");

        writer.writeBytes(magic);
        writer.writeUint32(1);

        let chunkStrokes: game.StrokeData[] = [];

        var cursorRequest = store!.index('chunk_key').openCursor(IDBKeyRange.bound(chunk_id, chunk_id));


        cursorRequest.onsuccess = function (e) {

            var cursor = (e as any).target.result;
            if (cursor) {
                let stroke = cursor.value as game.StrokeData;

                if (stroke.source == game.StrokeSource.User) {
                    if (chunk_id == stroke.chunk_key) {
                        chunkStrokes.push(stroke);
                    }
                }

                stroke = structuredClone(stroke);
                stroke.source = game.StrokeSource.Storage;

                store?.put(stroke);

                cursor.continue();
            }
            else {
                tx?.commit();
                writeStrokes(writer, chunk_id, chunkStrokes)
                resolve(writer.getBuffer());
            };
        }
    });

    return p;
}

function save_to_file() {
    const tx = db?.transaction(strokes, "readonly");
    const store = tx?.objectStore(strokes);
    var cursorRequest = store!.index('chunk_key').openCursor(null, 'next');

    var writer = new BinaryWriter();
    let magic = new TextEncoder().encode("draw");

    writer.writeBytes(magic);
    writer.writeUint32(1);

    let currentChunkKey: string | null = null;
    let chunkStrokes: game.StrokeData[] = [];

    cursorRequest.onsuccess = function (e) {

        var cursor = (e as any).target.result;
        if (cursor) {
            let stroke = cursor.value as game.StrokeData;

            if (currentChunkKey != stroke.chunk_key) {
                if (currentChunkKey != null) {

                    writeStrokes(writer, currentChunkKey, chunkStrokes)
                }

                currentChunkKey = stroke.chunk_key
                chunkStrokes = [];
            }

            chunkStrokes.push(stroke);
            cursor.continue();
        }
        else {

            if (currentChunkKey != null) {

                writeStrokes(writer, currentChunkKey, chunkStrokes)
            }

            let buffer = writer.getBuffer();

            postMessage({
                type: "prompt_save_file",
                data: {
                    name: "canvas.bin",
                    mime: 'application/octet-stream',
                    data: buffer
                }
            }, {
                transfer: [buffer]
            })
        }
    };

}

function writeStrokes(writer: BinaryWriter, currentChunkKey: string, chunkStrokes: game.StrokeData[]) {
    console.log("Writing chunk: ", currentChunkKey);
    console.log(chunkStrokes);
    writer.writeString(currentChunkKey);

    var ownerToStrokes: Map<string, game.StrokeData[]> = new Map();

    chunkStrokes.forEach((stroke) => {
        let id = stroke.owner_id;
        if (id == undefined) {
            id = "";
        }

        if (!ownerToStrokes.has(id)) {
            ownerToStrokes.set(id, [])
        }

        let array = ownerToStrokes.get(id)!;
        array.push(stroke);

        ownerToStrokes.set(id, array);
    });

    let keys = ownerToStrokes.keys().toArray();

    writer.writeUint32(keys.length);


    ownerToStrokes.keys().forEach((key) => {
        let strokes = ownerToStrokes.get(key)!;
        console.log(`Writing ${strokes.length} strokes from '${key}' to chunk ${currentChunkKey}`)

        if (key == "") {
            writer.writeUint8(0);
        } else {
            writer.writeUint8(1);
            writer.writeString(key);
        }

        writer.writeUint32(strokes.length);

        strokes.forEach((stroke) => {

            writeStroke(writer, stroke);
        });
    });
}



function writeStroke(writer: BinaryWriter, stroke: game.StrokeData) {
    writer.writeUint32(stroke.id_random);
    writer.writeFloat64(stroke.timestamp);
    writer.writeFloat32(stroke.origin_x);
    writer.writeFloat32(stroke.origin_y);
    writer.writeUint32(stroke.stroke_data.length);
    writer.writeBytes(stroke.stroke_data);
}

interface PendingChunkData {
    strokes: any,
    chunk_key: any,
    index_data: ArrayBuffer,
    vertex_data: ArrayBuffer,
    color_data: ArrayBuffer,
}

var chunk_queue = new AsyncTaskQueue<PendingChunkData>(append_chunk_data_task);

function append_chunk_data_task(data: PendingChunkData): Promise<void> {
    return new Promise(resolve => {

        console.log("Appending mesh data!", data.chunk_key);
        
        const tx = db?.transaction([strokes, mesh], "readwrite");

        tx!.onerror = (err) => {
            console.log("Transaction error: ", err);
        }

        console.log("Appending chunk data: ", data);
        let items = data.strokes;

        const store = tx?.objectStore(strokes);
        let meshes = tx?.objectStore(mesh);

        let mesh_data_request = meshes!.get(IDBKeyRange.only(data.chunk_key));

        var indices = new Uint32Array(data.index_data);
        var vertices = new Uint8Array(data.vertex_data);
        var colors = new Uint8Array(data.color_data);

        let meshData: any = {
            chunk_key: data.chunk_key,
            indices: indices,
            colors: colors,
            vertices: vertices,
        };

        items.forEach((data: game.StrokeData) => {
            store?.put({
                id: data.id,
                id_random: data.id_random,
                chunk_key: data.chunk_key,
                owner_id: data.owner_id,
                timestamp: data.timestamp,
                origin_x: data.origin_x,
                origin_y: data.origin_y,
                source: data.source,
                stroke_data: data.stroke_data,
            });
        });

        console.log("Initial mesh data: ", meshData);
        mesh_data_request.onsuccess = (ev) => {
            let mesh = (ev.target as IDBRequest).result;

            console.log("Got mesh: ", mesh);

            if (mesh != null) {
                var vertex_offset = mesh.vertices.length / (3 * 4);
                let arr = Uint32Array.from(indices);

                for (let i = 0; i < arr.length; i++) {
                    arr[i] = arr[i] + vertex_offset;
                }

                meshData = {
                    chunk_key: data.chunk_key,
                    indices: merge_arrays_uint32(mesh.indices, arr),
                    vertices: merge_arrays_uint8(mesh.vertices, vertices),
                    colors: merge_arrays_uint8(mesh.colors, colors),
                }
            }

            console.log("Inserting mesh data: ", meshData);

            meshes?.put(meshData)

            console.log("Posting new chunk data!");
            postMessage({
                type: "loaded_mesh_for_chunk",
                data: {
                    chunk_key: data.chunk_key,
                    vertex_data: meshData.vertices.buffer,
                    index_data: meshData.indices.buffer,
                    color_data: meshData.colors.buffer,
                }
            }, {
                transfer: [meshData.vertices.buffer, meshData.indices.buffer, meshData.colors.buffer]
            });

            tx?.commit();
            resolve();
        }

        mesh_data_request.onerror = (ev) => {
            tx?.commit();
            resolve();
        }
    });
}

function append_chunk_data(data: any) {
    chunk_queue.push({
        chunk_key: data.chunk_key,
        color_data: data.color_data,
        vertex_data: data.vertex_data,
        index_data: data.index_data,
        strokes: data.strokes,
    })
}


function delete_stroke(id: string) {
    console.log("Deleting stroke: ", id);
    const tx = db?.transaction([strokes, mesh], "readwrite");
    const strokeStore = tx?.objectStore(strokes);
    let request = strokeStore?.get(IDBKeyRange.only(id))
    request!.onsuccess = (ev) => {
        let result = (ev.target as IDBRequest).result as game.StrokeData;
        let chunk = result.chunk_key;

        if (result.source != game.StrokeSource.User && result.source != game.StrokeSource.Remote) {
            console.log("Cannot delete stroke that was not from the user");
            return;
        }

        console.log("Exists in chunk: ", chunk);
        console.log("Mesh starts at: ", result.vertex_offset);
        console.log("Num verts: ", result.num_verts);

        let meshes = tx!.objectStore(mesh);
        let mesh_request = meshes.get(IDBKeyRange.only(chunk));
        mesh_request.onsuccess = (ev) => {
            let mesh = (ev.target as IDBRequest).result as MeshData;
            console.log("Got mesh: ", mesh);
            let buf = mesh.vertices.buffer;
            let view = new Float32Array(buf);

            let start_index = result.vertex_offset! * 3;
            for (var i = start_index; i < start_index + (result.num_verts! * 3); i++) {
                view[i] = 0.0;
            }

            console.log("Replacing mesh data");
            let new_mesh: MeshData = {
                chunk_key: chunk,
                vertices: new Uint8Array(view.buffer),
                colors: mesh.colors,
                indices: mesh.indices,
            }

            let remove = strokeStore?.delete(IDBKeyRange.only(id))
            remove!.onsuccess = (_) => {
                console.log("Successfully removed mesh from db")
                let req = meshes.put(new_mesh);
                req.onsuccess = (_) => {
                    console.log("Successfully replaced mesh data")
                    postMessage({
                        type: "remove_verts",
                        data: {
                            chunk_key: result.chunk_key,
                            offset: result.vertex_offset,
                            num_verts: result.num_verts,
                        }
                    })
                }
            }
        }
    }
}

