import { createSignal, onMount, Show, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';

import * as game from '../bevy/draw-bevy';


import '@material/web/iconbutton/filled-icon-button';
import '@material/web/button/filled-button.js';
import '@material/web/checkbox/checkbox.js';
import App, { GameDelegate, NetworkDelegate } from '../organisms/app';
import { useSearchParams } from '@solidjs/router';

import { createMatrixRTCSdk, MatrixRTCSdk } from '../matrixrtc/matrixrtc-sdk.js';
import { INotifyCapabilitiesActionRequest, IVisibilityActionRequest, MatrixCapabilities, WidgetApi, WidgetApiFromWidgetAction } from 'matrix-widget-api';
import { UploadQueue } from '../utils/upload_queue';
import { useSaveProgress } from '..';
import { base64ToUint8, uint8ToBase64 } from '../utils/b64';



interface BackendChunk {
    sender: string,
    url: string,
    chunk: string,
}

class MatrixRTCDelegate implements NetworkDelegate {

    chunkEventType = "com.lagmachine.drawinggame.chunk";
    documentEventType = "com.lagmachine.drawinggame.document";

    sdk: MatrixRTCSdk
    on_ready: ((user_id: string) => void) | null;
    on_received: ((message: Uint8Array, from: string) => void) | null;
    on_peer_connected: ((from: string) => void) | null;
    on_peer_disconnected: ((from: string) => void) | null;

    documentId: string | null;

    requiredCapabilities: string[];
    uploadQueue: UploadQueue;

    pendingChunks: BackendChunk[];
    events: EventTarget;

    onConnectionStatusChanged = (status: any) => {
        console.log("Connection status changed: ", status);
    }

    onReceivedData = (data: any) => {
        console.log("Received data!", data);
        let from = data.rtcBackendIdentity;
        var bytes = base64ToUint8(JSON.parse(data.data));

        if (this.on_received != null) {
            this.on_received!(bytes, from);
        } else {
            console.log("No receipt callback!");
        }
    }

    connectedMembers: string[] = new Array();
    onMembersChanged = (data: any[]) => {
        console.log("Members changed!", data);
        console.log("Current: ", this.connectedMembers)
        for (var i = 0; i < data.length; i++) {
            var d = data[i];
            var id = d.membership.rtcBackendIdentity;

            if (this.connectedMembers.indexOf(id) == -1) {
                this.connectedMembers.push(id);
                console.log("Peer connected! ", id);

                if (this.on_peer_connected != null) {
                    this.on_peer_connected!(id);
                }
            }
        }


    }

    onLocalMembershipChanged = (data: any) => {
        console.log("Local membership changed: ", data);
        let localId = data.membership.rtcBackendIdentity
        console.log("Local member id: ", localId);
    }

    send_to(message: Uint8Array, to: string) {
        console.log("Attempting to send data ", message);
    }

    broadcast(message: Uint8Array) {
        const text = uint8ToBase64(message);
        this.sdk.sendData!(text);
    }

    constructor(sdk: MatrixRTCSdk) {
        this.sdk = sdk;
        this.on_peer_connected = null;
        this.on_peer_disconnected = null;
        this.on_ready = null;
        this.on_received = null;
        this.documentId = null;
        this.pendingChunks = [];
        this.events = new EventTarget();

        sdk.connected$.subscribe(this.onConnectionStatusChanged.bind(this));
        sdk.data$.subscribe(this.onReceivedData.bind(this));
        sdk.members$.subscribe(this.onMembersChanged.bind(this))
        sdk.localMember$.subscribe(this.onLocalMembershipChanged.bind(this));

        this.uploadQueue = new UploadQueue(
            async (item) => {

                console.log("Uploading data for chunk to matrix homeserver", item.id);

                var result = await this.api.uploadFile(item.file.buffer as ArrayBuffer);

                await this.api.sendRoomEvent(this.chunkEventType, {
                    "chunk": item.id,
                    "url": result.content_uri,
                    "m.relates_to": {
                        "event_id": this.documentId!,
                        "rel_type": "m.reference"
                    },
                })
            }
        );

        this.requiredCapabilities = [
            `org.matrix.msc2762.receive.state_event:${this.documentEventType}`,
            `org.matrix.msc2762.send.state_event:${this.documentEventType}`,
            `org.matrix.msc2762.send.event:${this.documentEventType}`,
            `org.matrix.msc2762.receive.event:${this.documentEventType}`,
            `org.matrix.msc2762.receive.event:${this.chunkEventType}`,
            `org.matrix.msc2762.send.event:${this.chunkEventType}`,
            'org.matrix.msc4039.upload_file',
        ]

    }
    download_chunks = async (chunk_id: string) => {
        console.log("Downloading chunks from matrix homeserver", chunk_id);
        console.log(this.pendingChunks);


        var urls = this.pendingChunks.filter((i) => i.chunk == chunk_id);
        this.pendingChunks = this.pendingChunks.filter((i) => i.chunk != chunk_id);

        for (var i = 0; i < urls.length; i++) {
            var file = urls[i];

            if (file.chunk == chunk_id) {
                console.log(file.url);

                var data = await this.api.downloadFile(file.url);
                var blob = data.file as Blob;
                var array = new Uint8Array(await blob.arrayBuffer());
                console.log()
                game.load_chunk(array, file.sender);
            }
        }


    };



    upload_chunk(chunk_id: string, data: Uint8Array) {
        console.log("Matrix delegate uploading chunk: ", chunk_id)
        console.log(data);

        this.uploadQueue.enqueue({
            id: chunk_id,
            file: data
        })

    };


    get api(): WidgetApi {
        return this.sdk.widget!.api;
    }

    onGameReady() {
        console.log("Matrix delegate received game ready signal");
        console.log(this.connectedMembers);

        for (var i = 0; i < this.connectedMembers.length; i++) {
            if (this.on_peer_connected != null) {
                this.on_peer_connected!(this.connectedMembers[i]);
            }
        }
    }

    init = async () => {
        this.api.on('action:notify_capabilities', this.onCapabilitiesChanged.bind(this));
        console.log("REQUESTING PERMISSIONS TO READ DOCUMENT!!");
        console.log("REQUESTING PERMISSIONS TO READ DOCUMENT!!");
        console.log("REQUESTING PERMISSIONS TO READ DOCUMENT!!");
        console.log("REQUESTING PERMISSIONS TO READ DOCUMENT!!");
        console.log("REQUESTING PERMISSIONS TO READ DOCUMENT!!");
        console.log("REQUESTING PERMISSIONS TO READ DOCUMENT!!");
        console.log("REQUESTING PERMISSIONS TO READ DOCUMENT!!");

        const urlParams = new URLSearchParams(window.location.search)
        console.log("URL PARAMS:");
        console.log(urlParams);

        var roomId = urlParams.get("roomId")!;

        console.log("Room id: ", roomId);

        if (!await this.canReadDocument()) {
            this.api!.requestCapabilityToReceiveState(this.documentEventType);
            this.api!.requestCapabilityToSendState(this.documentEventType);

            this.api!.requestCapabilityToSendEvent(this.documentEventType);
            this.api!.requestCapabilityToReceiveEvent(this.documentEventType);

            this.api!.requestCapabilityToReceiveEvent(this.chunkEventType);
            this.api!.requestCapabilityToSendEvent(this.chunkEventType);
            this.api!.requestCapability(MatrixCapabilities.MSC4039UploadFile);

            this.sdk.widget!.api.updateRequestedCapabilities();
        }
        // let state = await sdk!.widget!.api!.readStateEvents(documentEventType, 1, "", [roomId]);
    }


    onCapabilitiesChanged(event: CustomEvent<INotifyCapabilitiesActionRequest>) {
        console.log("Received capabilities changed notification", event);
        console.log("Received capabilities changed notification", event);
        console.log("Received capabilities changed notification", event);
        console.log("Received capabilities changed notification", event);
        console.log("Received capabilities changed notification", event);
        console.log("Received capabilities changed notification", event);
        console.log("Received capabilities changed notification", event);
        console.log("Received capabilities changed notification", event);
        console.log("Received capabilities changed notification", event);
        console.log("Received capabilities changed notification", event);
        console.log("Received capabilities changed notification", event);


        for (var i = 0; i < this.requiredCapabilities.length; i++) {
            if (event.detail.data.approved.indexOf(this.requiredCapabilities[i]) == -1) {
                console.log("DOES NOT HAVE APPROVED CAPABILITY: ", this.requiredCapabilities[i]);
                return;
            }
        }

        this.tryReadDocument();
    }

    canReadDocument() {
        for (var i = 0; i < this.requiredCapabilities.length; i++) {
            if (this.api.hasCapability(this.requiredCapabilities[i]) == false) {
                console.log("Does not have required capabilities!", this.requiredCapabilities[i]);
                return false;
            }
        }
    }

    tryReadDocument = async () => {
        if (this.documentId != null) return;

        console.log("READING DOCUMENT INFO!");

        let state = await this.api.readStateEvents(this.documentEventType, 1, "");
        console.log("RECEIEVED STATE EVENT:", state)

        if (state.length == 0) {
            var event = await this.api.sendRoomEvent(this.documentEventType, {});
            console.log("Sent event: ", event);

            this.documentId = event.event_id!;

            var stateEvent = await this.api.sendStateEvent(this.documentEventType, "", {
                "event_id": event.event_id!
            })
        } else {
            console.log(state);
            for (var i = 0; i < state.length; i++) {
                var ev = state[i];
                var documentId = ev.content["event_id"];
                this.documentId = documentId as string;
            }
        }


        if (this.documentId == null) throw "Could not find or create document id";

        let nextBatch: string | undefined = undefined;

        while (true) {
            console.log("Loading batch: ", nextBatch);
            var related = await this.api.readEventRelations(this.documentId, undefined, "m.reference", this.chunkEventType, 100, nextBatch);
            console.log("Got related events: ", related);


            for (var i = 0; i < related.chunk.length; i++) {
                var chunkEvent = related.chunk[i];
                var chunk = chunkEvent.content["chunk"];
                var url = chunkEvent.content["url"];
                var sender = chunkEvent.sender;

                this.pendingChunks.push({
                    chunk: chunk as string,
                    url: url as string,
                    sender,
                })
            }

            if (related.next_batch == null) {
                break;
            }

            nextBatch = related.next_batch;
        }

        console.log("Found related events: ", related);

        this.events.dispatchEvent(new Event("ready"));
        return true;
    }
}

const MatrixWidget: Component = () => {
    const [searchParams, setSearchParams] = useSearchParams();
    const [delegate, setDelegate] = createSignal<MatrixRTCDelegate | null>(null);
    const [gameDelegate, setGameDelegate] = createSignal<GameDelegate | null>(null);

    const [saveProgress, setSaveProgress] = useSaveProgress();

    onMount(async () => {
        let sdk = await createMatrixRTCSdk("com.lagmachine.drawinggame")
        sdk.join();

        let delegate = new MatrixRTCDelegate(sdk);

        delegate.events.addEventListener("ready", (ev) => {
            console.log("Delegate is ready!");
            setDelegate(delegate);
        })

        delegate.uploadQueue.addEventListener("queuechanged", (ev) => {
            console.log("Upload queue changed!");
            console.log(ev);

            let queueLength = (ev as CustomEvent).detail.length as number;

            if (queueLength > 0) {
                setSaveProgress("Uploading: " + queueLength.toString());
            } else {
                setSaveProgress("");
            }
        });

        setGameDelegate({
            on_ready: () => {
                console.log("Received ready signal from game!");
                delegate.onGameReady();
            }
        })

        await delegate.init();

    })

    return (
        <Show when={delegate() != null && gameDelegate() != null}>
            <App instance_id={searchParams.id as string} network_delegate={delegate() as NetworkDelegate} game_delegate={gameDelegate() as GameDelegate} />
        </Show>
    );
};

export default MatrixWidget;
