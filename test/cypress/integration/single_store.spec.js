describe("Ordo Single Store", () => {
  it("Test sync reducer", () => {
    cy.visit("/");

    cy.get("button").contains("singleStoreSync").click();
    cy.wait(400);
    cy.get("button").contains("Start").click();

    cy.get("h1").contains(10);
    cy.wait(500);
    cy.get("h1").contains(11);
    cy.wait(500);
    cy.get("h1").contains(10);
  });
});
