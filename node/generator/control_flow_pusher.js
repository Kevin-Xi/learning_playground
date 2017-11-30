'use strict';

function push(genF, done) {
    if (typeof done !== 'function') done = () => {};

    const gen = genF();
    recur(gen.next());  // todo: -> iter

    function recur(cur) {
        if (cur.done) return done();

        if (cur.value instanceof Promise) {
            cur.value.then(res => {
                recur(gen.next(res));
            }).catch(gen.throw.bind(gen));
        } else {
            recur(gen.next(cur.value));
        }
    }
}

function pushAll(genFs) {
    const total = genFs.length;
    let done = 0;
    return new Promise((res, rej) => {
        genFs.forEach(genF => {
            push(genF, () => {
                if (++done === total) res();
            });
        });
    });
}

module.exports = {
    push,
    pushAll
}