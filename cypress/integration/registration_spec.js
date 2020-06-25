import Chance from 'Chance';

const chance = new Chance();

describe('registration', function () {

    it('should create register an account', function () {
        let password = chance.string({length: 8});

        cy.visit('http://localhost:3001/registration/create');

        cy.get('[name="first_name"').type(chance.first());
        cy.get('[name="last_name"').type(chance.last());
        cy.get('[name="email"').type(chance.email());
        cy.get('[name="password"').type(password);
        cy.get('[name="confirm_password"').type(password);
        cy.get('#primary-form-submit').click();

        cy.url().should('match', /\/registration\/confirmation-sent/)
    });

    it('should successfully confirm an account', function () {
        cy.visit('http://localhost:3001/registration/confirmation/f0168e2a-0162-46e0-a433-0e9a130a31cf');

        cy.get('h1').invoke('text').then((text) => {
            assert.equal(text.trim(), "Your email has been Confirmed!")
        });
    });

    it('should show an error message when a confirmation link is expired', function () {
        cy.visit('http://localhost:3001/registration/confirmation/f0168e2a-0162-46e0-a433-0e9a130a32cf');

        cy.get('h1').invoke('text').then((text) => {
            assert.equal(text.trim(), "Email Confirmation Link Expired")
        });
    });

    it('should allow a user to resend a confirmation email', function () {
        cy.visit('http://localhost:3001/registration/confirmation-create');

        cy.get('[name="email"').type("resend-confirmed@gmail.com");
        cy.get('#primary-form-submit').click();

        cy.get('.alert').invoke('text').then((text) => {
            assert.equal(text.trim(), "An email has been sent to confirm your email address")
        });
    });
});