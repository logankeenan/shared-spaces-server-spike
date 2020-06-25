import Chance from 'Chance';

const chance = new Chance();

describe('password', function () {
    it('should allow a user to reset their password', function () {
        cy.visit('http://localhost:3001/password/reset');
        cy.get('[name="email"').type(chance.email());

        cy.get('#primary-form-submit').click();

        cy.url().should('match', /\/password\/reset/);
        cy.get('.alert-success').invoke('text').then((text) => {
            assert.equal(text.trim(), "An email has been sent to reset your password")
        });
    });
});