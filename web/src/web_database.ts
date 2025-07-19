import * as game from './bevy/draw-bevy';

const strokes = "stroke"

export class WebDatabase {
    constructor() {
        this.db = null;
    }

    db: IDBDatabase | null


    clear(): Promise<void> {
        return new Promise(resolve => {
            var request = indexedDB.deleteDatabase("strokeStorage");

            request.onsuccess = event => {
                console.log("Cleared database!");
                resolve();
            }
        });
    }

    init(): Promise<void> {
        return new Promise(resolve => {
            var request = window.indexedDB.open("strokeStorage", 3);

            request.onupgradeneeded = event => {
                var db = (event.target as IDBOpenDBRequest).result;

                var objectStore = db.createObjectStore(strokes, {
                    keyPath: "id"
                })

                objectStore.createIndex("chunk_key", "chunk_key", { unique: false });
                objectStore.createIndex("owner_id", "owner_id", { unique: false });
            }

            request.onsuccess = event => {
                console.log("Opened database!");
                this.db = (event.target as IDBOpenDBRequest).result;

                resolve();
            }
        });
    }

    store_stroke(data: game.StrokeData) {
        const item = {
            id: data.id,
            id_random: data.id_random,
            chunk_key: data.chunk_key,
            owner_id: data.owner_id,
            timestamp: data.timestamp,
            origin_x: data.origin_x,
            origin_y: data.origin_y,
            stroke_data: data.stroke_data,
            mesh_data: data.mesh_data,
        };

        const tx = this.db?.transaction(strokes, "readwrite");
        const store = tx?.objectStore(strokes);
        store!.add(item);
        console.log("Stored stroke");

        data.free();
    }

    store_multiple_strokes(items: [game.StrokeData]) {
        const tx = this.db?.transaction(strokes, "readwrite");
        const store = tx?.objectStore(strokes);
        console.log("Inserting strokes: ", items.length);
        items.forEach((data) => store?.add({
            id: data.id,
            id_random: data.id_random,
            chunk_key: data.chunk_key,
            owner_id: data.owner_id,
            timestamp: data.timestamp,
            origin_x: data.origin_x,
            origin_y: data.origin_y,
            stroke_data: data.stroke_data,
            mesh_data: data.mesh_data,
        }))


        items.forEach((data) => data.free());
        console.log("Done!");
    }

    load_strokes_for_chunk(id: string) {
        const tx = this.db?.transaction(strokes, "readonly");
        const store = tx?.objectStore(strokes);
        const index = store?.index("chunk_key");
        var request = index?.getAll(IDBKeyRange.only(id))

        request!.onsuccess = (event) => {
            let result = (event.target as IDBRequest).result;

            if (result.length > 0) {

                let results = result.map((e: any) => new game.StrokeData(
                    e.id, e.id_random, e.chunk_key, e.timestamp, e.origin_x, e.origin_y, e.owner_id, e.stroke_data, e.mesh_data
                ));

                game.db_on_strokes_loaded(results, id);

                // results.forEach((e: any) => (e as game.StrokeData).free())

            }
        }

    }
}