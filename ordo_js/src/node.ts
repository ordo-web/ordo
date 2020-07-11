import {Node} from "ordo-core";

export class Class {
    node: Node;
    constructor(worker: Worker) {
        this.node = new Node(worker);
    }
}