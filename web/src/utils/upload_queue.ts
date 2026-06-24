interface PendingUpload {
    id: string;
    file: Uint8Array;
}

export class UploadQueue {
    private queue: PendingUpload[] = [];
    private processing = false;

    constructor(
        private readonly uploadFn: (upload: PendingUpload) => Promise<void>
    ) { 
    }

    enqueue(upload: PendingUpload): void {
        this.queue.push(upload);

        if (!this.processing) {
            void this.processQueue();
        }
    }

    private async processQueue(): Promise<void> {
        this.processing = true;

        try {
            while (this.queue.length > 0) {
                
                const upload = this.queue.shift()!;

                try {
                    await this.uploadFn(upload);
                } catch (error) {
                    console.error(
                        `Upload failed for ${upload.id}:`,
                        error
                    );
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