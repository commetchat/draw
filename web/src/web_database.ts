import * as game from './bevy/draw-bevy';
import { MeshData } from './bevy/draw-bevy';
import { NetworkDelegate } from './organisms/app';
import { downloadBlob } from './utils/download';
import Worker from './worker?worker';


export class WebDatabase {

    constructor(instance_id: string, delegate: NetworkDelegate) {
        this.db = null;
        this.worker = new Worker()
        this.instance_id = instance_id;
        this.worker.onmessage = this.handleMessage.bind(this);
        this.network_delegate = delegate;
    }

    instance_id: string
    db: IDBDatabase | null
    worker: Worker
    network_delegate: NetworkDelegate


    init() {
        this.worker.postMessage({
            type: "init_db",
            data: this.instance_id,
        });
    }


    store_multiple_strokes(items: [game.StrokeData], source: string) {
        let converted_items = items.map((a) => {

            return this.convertStrokeDataToJs(a)
        })

        this.worker.postMessage({
            type: "store_multiple_strokes",
            data: converted_items,
            source: source,
        })
    }

    append_chunk_data(chunk_key: string, vertex_data: Uint8Array, index_data: Uint32Array, color_data: Uint8Array, strokes: [game.StrokeData]) {
        let converted_strokes = strokes.map((a) => {
            return this.convertStrokeDataToJs(a)
        })

        this.worker.postMessage({
            type: "append_chunk_data",
            data: {
                chunk_key: chunk_key,
                vertex_data: vertex_data.buffer,
                index_data: index_data.buffer,
                color_data: color_data.buffer,
                strokes: converted_strokes,
            }
        }, [vertex_data.buffer, index_data.buffer, color_data.buffer])
    }

    send_strokes_to_user(identifier: String) {
        this.worker.postMessage({
            type: "send_strokes_to_user",
            data: identifier,
        })
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
            source: a.source,
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


    saveToBackend() {
        this.worker.postMessage({
            type: "save_to_backend"
        })
    }

    delete_stroke(id: string) {
        this.worker.postMessage({
            type: "delete_stroke",
            data: id
        })
    }


    handleMessage(message: MessageEvent<any>) {
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

        if(message.data.type == "alert") {
            window.alert(message.data.data)
        }

        if (message.data.type == "send_to_user") {
            let bytes = new Uint8Array(message.data.data.data); //LOL
            let user = message.data.data.user as string;
            this.network_delegate.send_to(bytes, user)
        }

        if (message.data.type == "remove_verts") {
            game.db_remove_verts(message.data.data.chunk_key, message.data.data.offset, message.data.data.num_verts)
        }

        if(message.data.type == "save_chunk") {
            console.log("SAVE CHUNK TO BACKEND");
            console.log(message.data);
            let bytes = message.data.data.data as ArrayBuffer;
            this.network_delegate.upload_chunk!(message.data.data.chunk_id, new Uint8Array(bytes));
        }
    }
}

function handle_mesh_loaded(data: any) {
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
            s.color_data,
            s.source,
        )
    });

    game.db_append_mesh_data(converted);
}

