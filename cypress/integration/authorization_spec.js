import Chance from 'Chance';

const chance = new Chance();

describe('authorization', function () {
    it('redirect to the login page if they have not logged in', function () {
        cy.request({method: 'GET', url: 'http://localhost:3001/users/1', failOnStatusCode: false})
            .then((response) => {
                expect(response.status).to.equal(401)
            })
    });
});