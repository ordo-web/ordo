describe("Ordo Single Store", () => {
  it("Test sync reducer", () => {
    cy.visit("/");

    cy.get("button").click();
    //cy.wait(10);
    cy.get("h1").contains(10);

    cy.wait(500);
    cy.get("h1").contains(11);

    cy.wait(500);
    cy.get("h1").contains(10);
  });
});
