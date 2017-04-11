'use strict';

const User = require('./User');

function main() {
    let user = new User('kevin');

    return {getName: user.getName(), name: user.name};
}

module.exports = main;
