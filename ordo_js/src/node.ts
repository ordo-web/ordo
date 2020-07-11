import {Node as PrimeNode} from "ordo-core";

export class Node {
    private node: PrimeNode;
    private readonly worker: Worker;
    constructor(worker: Worker) {
        this.worker = worker;
        this.node = new PrimeNode(this.worker);
    }
}

