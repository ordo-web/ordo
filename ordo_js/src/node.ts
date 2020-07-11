import {Node as RepNode } from "ordo-core";

export class Node {
    node: RepNode;
    constructor(worker: Worker) {
        this.node = new RepNode(worker);
    }
}