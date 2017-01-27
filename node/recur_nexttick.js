'use strict';
const fs = require('fs');

function doSomething () {
    process.nextTick(doSomething);
}

process.nextTick(doSomething);

fs.access('.', console.info);
