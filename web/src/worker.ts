import * as game from './bevy/draw-bevy';

let db: IDBDatabase | null

self.onmessage = function (e) {
    if (e.data.type == "init_db") {
        //  console.log = (e) => {
        //      postMessage({
        //          type: "log",
        //          data: e,
        //      })
        //  }
        initDb();
    }

    if (e.data.type == "store_multiple_strokes") {
        store_multiple_strokes(e.data.data);
    }

    if (e.data.type == "load_mesh_for_chunk") {
        load_mesh_for_chunk(e.data.data);
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

function clearDb(): Promise<void> {
    console.log("Clearing database");
    return new Promise(resolve => {
        var request = indexedDB.deleteDatabase("strokeStorage");
        console.log("Created request");


        resolve();

    });
}

function initDb() {
    console.log("Initializing database");
    clearDb().then(() => {
        console.log("Creating database");
        var request = indexedDB.open("strokeStorage", 3);

        request.onupgradeneeded = event => {
            console.log("Upgrade needed");
            var db = (event.target as IDBOpenDBRequest).result;

            var objectStore = db.createObjectStore(strokes, {
                keyPath: "id"
            })

            objectStore.createIndex("chunk_key", "chunk_key", { unique: false });
            objectStore.createIndex("owner_id", "owner_id", { unique: false });

            var mesh_store = db.createObjectStore(mesh, {
                keyPath: "chunk_key"
            });
        }

        request.onsuccess = event => {
            console.log("Opened database!");
            db = (event.target as IDBOpenDBRequest).result;
            self.postMessage({ type: "db_init" });
        }

        request.onerror = event => {
            console.log("Failed to open database!");
        }
    });
}


function store_multiple_strokes(items: [game.StrokeData]) {
    const tx = db?.transaction([strokes, mesh], "readwrite");
    tx!.oncomplete = () => {
        console.log("Transaction complete!")
    }

    tx!.onerror = (err) => {
        console.log("Transaction error: ", err);
    }

    const store = tx?.objectStore(strokes);
    console.log("Inserting strokes: ", items.length);


    let keys = new Set<String>()

    items.forEach((e) => {
        keys.add(e.chunk_key);
    });

    let chunk_keys = keys.keys().toArray().sort();

    let meshes = tx?.objectStore(mesh);
    console.log("Keys: ", chunk_keys)
    let mesh_data_request = meshes!.getAll(IDBKeyRange.bound(chunk_keys[0], chunk_keys[chunk_keys.length - 1]));

    mesh_data_request.onsuccess = (ev) => {
        let result = (ev.target as IDBRequest).result as MeshData[];

        let mesh_map: Map<string, MeshData> = new Map();

        result.forEach((mesh) => {
            mesh_map.set(mesh.chunk_key, mesh);
        });

        items.forEach((data) => {

            let mesh = mesh_map.get(data.chunk_key);

            store?.add({
                id: data.id,
                id_random: data.id_random,
                chunk_key: data.chunk_key,
                owner_id: data.owner_id,
                timestamp: data.timestamp,
                origin_x: data.origin_x,
                origin_y: data.origin_y,
                stroke_data: data.stroke_data,
            });

            if (mesh == null) {
                mesh = {
                    chunk_key: data.chunk_key,
                    indices: data.index_data,
                    colors: data.color_data,
                    vertices: data.vertex_data,
                };
            } else {

                let num_points = mesh.vertices.length / (3 * 4);
                let arr = Uint32Array.from(data.index_data);

                for (let i = 0; i < arr.length; i++) {
                    arr[i] = arr[i] + num_points;

                }

                mesh = {
                    chunk_key: data.chunk_key,
                    indices: merge_arrays_uint32(mesh.indices, arr),
                    vertices: merge_arrays_uint8(mesh.vertices, data.vertex_data),
                    colors: merge_arrays_uint8(mesh.colors, data.color_data),
                }
            }

            mesh_map.set(data.chunk_key, mesh);
        })

        mesh_map.values().forEach((e) => meshes!.put(e));

        tx?.commit();
    }

    mesh_data_request.onerror = (ev) => {
        console.log("Failed to get mesh from db")
    }


    //items.forEach((data) => data.free());
    console.log("Done!");
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

function merge_arrays_uint8(a: Uint8Array, b: Uint8Array): Uint8Array {
    var mergedArray = new Uint8Array(a.length + b.length);
    mergedArray.set(a);
    mergedArray.set(b, a.length);

    return mergedArray;
}

function merge_arrays_uint32(a: Uint32Array, b: Uint32Array): Uint32Array {
    var mergedArray = new Uint32Array(a.length + b.length);
    mergedArray.set(a);
    mergedArray.set(b, a.length);

    return mergedArray;
}