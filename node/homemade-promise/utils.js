'use strict';

function promisify(fn, ...arg) {
    return new Promise((resolve, reject) => {
        fn(...arg, (err, result) => {
            if (err) reject(err);
            else resolve(result);
        });
    });
}

module.exports = {
    promisify
}