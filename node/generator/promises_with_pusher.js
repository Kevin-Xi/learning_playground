'use strict';

const fs = require('fs');
const push = require('./control_flow_pusher');
const promiseUtils = require('../homemade-promise/utils');

let readArgP = () => {
    process.stdin.setEncoding('utf8');

    return new Promise((resolve, reject) => {
        process.stdin.on('readable', () => {
            let chunk = process.stdin.read();
            if (chunk !== null) {
                let filename = chunk.trim() || process.argv[1];
                resolve(filename);
            }
        });
    });
}

let readFileP = (filename, opt) => promiseUtils.promisify(fs.readFile, filename, opt);

let printLineByLineP = file => {
    return new Promise((resolve, reject) => {
        let left = 0;
        file.split('\n').forEach((line, i) => {
            ++left;
            setTimeout(l => {
                console.log(l);
                if (--left === 0) resolve();
            }, i * 50, line);
        });
    });
}

push(function* () {
    process.stdout.write(`filename (default is self) > `);
    let filename = yield readArgP();
    console.log(`try to read ${filename}`);

    let file = '';
    try {
        file = yield readFileP(filename, {encoding: 'utf8'});
    } catch (e) {
        console.error(e.stack || e);
        process.exit(-1);
    }

    yield printLineByLineP(file);

    process.exit(0);
});