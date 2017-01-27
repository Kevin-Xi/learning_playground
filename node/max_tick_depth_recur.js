'use strict';
const fs = require('fs');

let i = 0;
function recur(i) {
    if (i === 1e5) return;
    // console.info(`begin ${++i}`);
    fs.access(`.`, (err, result) => {
        process.stdout.write('.');
        process.nextTick(recur, i+1);
    });
}

recur(i);
