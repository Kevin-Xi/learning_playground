'use strict';

module.exports = function push(fn) {
    let gen = fn();
    let next = gen.next();
    loop(next, gen);
}

function loop(next, gen) {
    if (next.done) return;

    if (next.value instanceof Promise) {
        next.value.then(val => {
            loop(gen.next(val), gen);
        }).catch(err => {
            gen.throw(err);
        });
    } else {
        loop(gen.next(next.value), gen);
    }
}