import { sum } from "../src/sum";
import { expect } from "chai";
import "mocha";

describe("Sum Test", () => {
  it("should return true", () => {
    const result = sum(1, 2);
    expect(result).to.equal(3);
  });
});
