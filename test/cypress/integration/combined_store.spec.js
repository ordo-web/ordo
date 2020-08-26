describe("Ordo Combined Store", () => {
  it("Test with sync reducer", () => {
    cy.visit("/");

    cy.get("button").contains("combinedStoreSync").click();
    cy.wait(550);
    cy.get("button").contains("Start").click();

    cy.get("h1").eq(0).should("be.empty");
    cy.wait(500);
    cy.get("h1").eq(0).contains(10);
    cy.wait(500);
    cy.get("h1").eq(0).should("be.empty");
    cy.wait(300);
    cy.get("h1").eq(1).contains(100);
    cy.wait(300);
    cy.get("h1").eq(1).contains(1000);
    cy.wait(300);
    cy.get("h1").eq(1).contains(100);
  });

  it("Test with async reducer", () => {
    cy.visit("/");

    cy.get("button").contains("combinedStoreAsync").click();
    cy.wait(550);
    cy.get("button").contains("Start").click();

    cy.get("h1").eq(0).should("be.empty");
    cy.wait(500);
    cy.get("h1").eq(0).contains(10);
    cy.wait(500);
    cy.get("h1").eq(0).should("be.empty");
    cy.wait(300);
    cy.get("h1").eq(1).contains(100);
    cy.wait(300);
    cy.get("h1").eq(1).contains(1000);
    cy.wait(300);
    cy.get("h1").eq(1).contains(100);
  });

  /**
  it("Test Single Store with dispatches from prime node", () => {
    cy.visit("/");

    cy.get("button").contains("singleStoreWorker").click();
    cy.wait(550);
    cy.get("button").contains("Start").click();

    cy.get("h1").contains("Hello!");
    cy.wait(500);
    cy.get("h1").contains("Hello World!");
    cy.wait(500);
    cy.get("h1").contains("Hello!");
  });*/
});
