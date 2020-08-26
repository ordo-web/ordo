describe("Ordo Single Store", () => {
  it("Test with sync reducer", () => {
    cy.visit("/");

    cy.get("button").contains("singleStoreSync").click();
    cy.wait(550);
    cy.get("button").contains("Start").click();

    cy.get("h1").contains(10);
    cy.wait(500);
    cy.get("h1").contains(11);
    cy.wait(500);
    cy.get("h1").contains(10);
  });

  it("Test with async reducer", () => {
    cy.visit("/");

    cy.get("button").contains("singleStoreAsync").click();
    cy.wait(550);
    cy.get("button").contains("Start").click();

    cy.get("h1").contains("Hello!");
    cy.wait(500);
    cy.get("h1").contains("Hello World!");
    cy.wait(500);
    cy.get("h1").contains("Hello!");
  });

  it("Test with dispatches from prime node", () => {
    cy.visit("/");

    cy.get("button").contains("singleStoreWorker").click();
    cy.wait(550);
    cy.get("button").contains("Start").click();

    cy.get("h1").contains("Hello!");
    cy.wait(500);
    cy.get("h1").contains("Hello World!");
    cy.wait(500);
    cy.get("h1").contains("Hello!");
  });
});
