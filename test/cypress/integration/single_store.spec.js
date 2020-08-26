describe("Ordo", () => {
  it("Test Single Store with sync reducer", () => {
    cy.visit("/");

    cy.get("button").contains("singleStoreSync").click();
    cy.wait(500);
    cy.get("button").contains("Start").click();

    cy.get("h1").contains(10);
    cy.wait(500);
    cy.get("h1").contains(11);
    cy.wait(500);
    cy.get("h1").contains(10);
  });

  it("Test Single Store with async reducer", () => {
    cy.visit("/");

    cy.get("button").contains("singleStoreAsync").click();
    cy.wait(500);
    cy.get("button").contains("Start").click();

    cy.get("h1").contains("Hello!");
    cy.wait(500);
    cy.get("h1").contains("Hello World!");
    cy.wait(500);
    cy.get("h1").contains("Hello!");
  });
});
