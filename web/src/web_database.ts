import * as game from './bevy/draw-bevy';
import { MeshData } from './bevy/draw-bevy';
import Worker from './worker?worker';


export class WebDatabase {
    constructor() {
        this.db = null;
        this.worker = new Worker()

        this.worker.onmessage = handleMessage
    }

    db: IDBDatabase | null
    worker: Worker


    init() {
        this.worker.postMessage({
            type: "init_db",
        });
    }

    store_multiple_strokes(items: [game.StrokeData]) {

        let converted_items = items.map((a) => {
            return {
                id: a.id,
                id_random: a.id_random,
                timestamp: a.timestamp,
                chunk_key: a.chunk_key,
                origin_x: a.origin_x,
                origin_y: a.origin_y,
                stroke_data: a.stroke_data,
                vertex_data: a.vertex_data,
                index_data: a.index_data,
                color_data: a.color_data,
            }
        })

        this.worker.postMessage({
            type: "store_multiple_strokes",
            data: converted_items
        })
    }

    load_mesh_for_chunk(id: string) {
        this.worker.postMessage({
            type: "load_mesh_for_chunk",
            data: id
        })
    }
}

function handleMessage(message: MessageEvent<any>) {
    if (message.data.type == "loaded_mesh_for_chunk") {
        handle_mesh_loaded(message.data.data);
    }
}

function handle_mesh_loaded(data: MeshData) {
    console.log("Received mesh data from worker!");
    game.db_on_mesh_loaded(new game.MeshData(data.chunk_key, data.vertex_data, data.index_data, data.color_data));
}

