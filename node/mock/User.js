'use strict';

module.exports = class User {
    constructor(name) {
        this.user_name = name;
    }

    get name() {
        return this.user_name;
    }

    getName() {
        return this.user_name;
    }
}
