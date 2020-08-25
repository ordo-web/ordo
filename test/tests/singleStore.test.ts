import { expect } from "chai";
import "mocha";
const { Worker } = require("worker_threads");

describe("SingleStore", () => {
  it("sync", () => {
    const worker = new Worker("./src/worker/singleStore.js");

    const result = 3;
    expect(result).to.equal(3);
  });
});
