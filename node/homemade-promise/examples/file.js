'use strict';
const fs = require('fs');
const Promise = require('../');

let readP = (path, opt) => {
    let p = new Promise();
    fs.readFile(path, opt, (err, file) => {
        if (err) p.reject(err);
        p.resolve(file);
    });

    return p;
};

let lineByLineP = file => {
    let p = new Promise();
    let left = 0;
    file.toString().split('\n').forEach((line, i) => {
        left++;
        setTimeout(() => {
            console.log(line);
            if (--left === 0) p.resolve(i);
        }, i * 50);
    });

    return p;
};

let done = hint => {
    let p = new Promise();
    console.log(hint);
    setImmediate(() => p.resolve());
    return p;
};



readP('./index.js')
    .then(lineByLineP)
    .then(done)
    .then(() => readP('./examples/file.js'))
    .then(lineByLineP)
    .then(done)
    .then(() => 'all finish')
    .then(console.log);
