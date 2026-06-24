interface PendingUpload {
    id: string;
    file: Uint8Array;
}

export class UploadQueue {
    private queue: PendingUpload[] = [];
    private processing = false;
    private events = new EventTarget();

    constructor(
        private readonly uploadFn: (upload: PendingUpload) => Promise<void>
    ) {
    }

    enqueue(upload: PendingUpload): void {
        this.queue.push(upload);
        this.alertQueueChanged();

        if (!this.processing) {
            this.processQueue();
        }
    }

    addEventListener(type: string, callback: EventListenerOrEventListenerObject | null) {
        this.events.addEventListener(type, callback);
    }

    alertQueueChanged() {
        var ev = new CustomEvent("queuechanged", {
            detail: {
                length: this.queue.length,
            }
        });

        this.events.dispatchEvent(ev);
    }

    private async processQueue(): Promise<void> {
        this.processing = true;

        try {
            while (this.queue.length > 0) {

                const upload = this.queue.shift()!;

                try {
                    await this.uploadFn(upload);
                    this.alertQueueChanged();
                } catch (error) {
                    console.error(
                        `Upload failed for ${upload.id}:`,
                        error
                    );
                    this.alertQueueChanged();
                }
            }
        } finally {
            this.processing = false;


            if (this.queue.length > 0) {
                void this.processQueue();
            }
        }
    }
}