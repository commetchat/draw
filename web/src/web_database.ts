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
        console.log("Storing multiple strokes: ")
        console.log(items);

        let converted_items = items.map((a) => {
            return this.convertStrokeDataToJs(a)
        })

        this.worker.postMessage({
            type: "store_multiple_strokes",
            data: converted_items
        })
    }

    set_initial_chunk_state(chunk_key: string, vertex_data: Uint8Array, index_data: Uint32Array, color_data: Uint8Array, strokes: [game.StrokeData]) {
        let converted_strokes = strokes.map((a) => {
            return this.convertStrokeDataToJs(a)
        })

        this.worker.postMessage({
            type: "set_initial_chunk_state",
            data: {
                chunk_key: chunk_key,
                vertex_data: vertex_data.buffer,
                index_data: index_data.buffer,
                color_data: color_data.buffer,
                strokes: converted_strokes,
            }
        }, [vertex_data.buffer, index_data.buffer, color_data.buffer])
    }

    private convertStrokeDataToJs(a: game.StrokeData): any {
        let result = {
            id: a.id,
            id_random: a.id_random,
            timestamp: a.timestamp,
            owner_id: a.owner_id,
            chunk_key: a.chunk_key,
            origin_x: a.origin_x,
            origin_y: a.origin_y,
            vertex_offset: a.vertex_offset,
            num_verts: a.num_verts,
            stroke_data: a.stroke_data,
            vertex_data: a.vertex_data,
            index_data: a.index_data,
            color_data: a.color_data,
        };

        a.free()

        return result;
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

    if (message.data.type == "do_full_chunk_reload") {
        game.db_chunk_needs_reloading(message.data.data.chunk_key);
    }

    if (message.data.type == "append_mesh_data") {
        append_mesh_data(message.data.data);
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

function append_mesh_data(data: game.StrokeData[]) {
    let converted = data.map((s) => {
        return new game.StrokeData(
            s.id,
            s.id_random,
            s.chunk_key,
            s.timestamp,
            s.origin_x,
            s.origin_y,
            s.owner_id,
            s.stroke_data,
            s.vertex_offset,
            s.num_verts,
            s.vertex_data,
            s.index_data,
            s.color_data
        )
    });



    game.db_append_mesh_data(converted);
}

