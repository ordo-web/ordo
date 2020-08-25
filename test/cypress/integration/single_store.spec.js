describe("Ordo Single Store", () => {
  it("Test sync reducer", () => {
    cy.visit("/");

    cy.get("h2").eq(10);

    cy.wait(3000);
    cy.get("h2").eq(11);

    cy.wait(3000);
    cy.get("h2").eq(10);
  });
});
