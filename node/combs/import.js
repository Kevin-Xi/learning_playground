'use strict';

const combs = require('combinators-js');

for (let n in combs) {
    global[n] = combs[n];
}

console.log(B.toString());
