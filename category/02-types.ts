'use strict';

const memoize: <T, U>(f: (x: T) => U) => (x: T) => U = f => {
    const memo = new Map();
    return x => {
        if (memo.has(x)) {
            return memo.get(x);
        } else {
            let value = f(x);
            memo.set(x, value);
            return value;
        }
    }
}

let m = memoize(Math.random);
console.log(m(1));
console.log(m(2));
console.log(m(1));
console.log(m(2));

let add1: (a: number) => number = a => a + 1;
let n = memoize(add1);
console.log(n(1));
console.log(n(2));
console.log(n(1));
console.log(n(2));