export class AsyncTaskQueue<T> {
    private queue: T[] = new Array();
    private running: boolean;
    private events = new EventTarget();

    process: (value: T) => Promise<void>;

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

    constructor(process: (value: T) => Promise<void>) {
        this.queue = new Array();
        this.running = false;
        this.process = process;
    }

    push(value: T) {
        this.queue.push(value);
        this.alertQueueChanged()

        if (this.running == false) {
            this.runQueue();
        }
    }

    async runQueue() {
        this.running = true;

        let item = this.queue.shift();

        while (item != undefined) {
            try {
                await this.process(item);
                this.alertQueueChanged()
            } catch (e) {
                console.log("Hit error while running queue: ", e);
                this.push(item);
            }

            item = this.queue.shift();
        }


        this.running = false;
    }
}