import * as game from './bevy/draw-bevy';
import { MeshData } from './bevy/draw-bevy';
import { downloadBlob } from './utils/download';
import Worker from './worker?worker';


export class WebDatabase {

    constructor(instance_id: string) {
        this.db = null;
        this.worker = new Worker()
        this.instance_id = instance_id;
        this.worker.onmessage = handleMessage
    }

    instance_id: string
    db: IDBDatabase | null
    worker: Worker


    init() {
        this.worker.postMessage({
            type: "init_db",
            data: this.instance_id,
        });
    }


    store_multiple_strokes(items: [game.StrokeData]) {

        let converted_items = items.map((a) => {
            return {
                id: a.id,
                id_random: a.id_random,
                timestamp: a.timestamp,
                owner_id: a.owner_id,
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

    save_to_file() {
        this.worker.postMessage({
            type: "save_to_file",
        });
    }

}

function handleMessage(message: MessageEvent<any>) {
    if (message.data.type == "loaded_mesh_for_chunk") {
        handle_mesh_loaded(message.data.data);
    }

    if (message.data.type == "db_init") {
        handle_database_ready();
    }

    if (message.data.type == "log") {
        console.log(message.data.data);
    }

    if (message.data.type == "prompt_save_file") {
        prompt_save_file(message.data.data);
    }
}

function handle_mesh_loaded(data: any) {
    console.log("Received mesh data from worker!");
    let verts = new Uint8Array(data.vertex_data);
    let indices = new Uint32Array(data.index_data);
    let colors = new Uint8Array(data.color_data);
    game.db_on_mesh_loaded(new game.MeshData(data.chunk_key, verts, indices, colors));
}

function handle_database_ready() {
    console.log("Database is ready!");
    game.db_ready();
}


function prompt_save_file(data: any) {
    downloadBlob(new Uint8Array(data.data), data.name, data.mime);
}

