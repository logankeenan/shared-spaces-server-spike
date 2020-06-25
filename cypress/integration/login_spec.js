import Chance from 'Chance';

const chance = new Chance();

describe('login', function () {
    function logInExistingUser() {
        cy.visit('http://localhost:3001/session/create');
        cy.get('[name="email"').type("login-success@gmail.com");
        cy.get('[name="password"').type("123456789");
        cy.get('#primary-form-submit').click();
    }

    it('should allow an existing user to login', function () {
        logInExistingUser();

        cy.url().should('match', /.*\/users\/1/);
    });

    it('should show an error message when the email and/or password is invalid', function () {
        cy.visit('http://localhost:3001/session/create');

        cy.get('[name="email"').type(chance.email());
        cy.get('[name="password"').type(chance.string({length: 8}));
        cy.get('#primary-form-submit').click();

        cy.get('.alert').invoke('text').then((text) => {
            assert.equal(text.trim(), "Invalid password or email")
        });
    });

    it('should show a message to confirm the account when the account has not been confirmed yet', function () {
        cy.visit('http://localhost:3001/session/create');
        cy.get('[name="email"').type("login-unconfirmed@gmail.com");
        cy.get('[name="password"').type("123456789");
        cy.get('#primary-form-submit').click();

        cy.url().should('match', /\/registration\/confirmation-please/);
        cy.get('h1').invoke('text').then((text) => {
            assert.equal(text.trim(), "Please Confirm your Email")
        });
    });

    it('should allow a user to logout', function () {
        logInExistingUser();

        cy.visit('http://localhost:3001/users/1');

        cy.get('#logout-btn').click();

        cy.request({method: 'GET', url: 'http://localhost:3001/users/1', failOnStatusCode: false})
            .then((response) => {
                expect(response.status).to.equal(401)
            });
    });
});