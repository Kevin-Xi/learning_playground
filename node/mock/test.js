'use strict';

const app = require('./app');

(function normalTest() {
    console.info(app());
})();

(function mockedTest() {
    const User = require('./User');
    // console.info(Object.getOwnPropertyDescriptor(User.prototype, 'getName'));
    User.prototype.getName = () => 'mocked';

    console.info(app());
})();
