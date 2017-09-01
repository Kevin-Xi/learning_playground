'use strict';
import assert = require("assert");

/* prelude */
const id: <T>(m: T) => T = x => x
const compose: <T, U, V>(f: (a: T) => U) => (g: (b: U) => V) => (x: T) => V
     = f => g => x => g(f(x));

/* prelude end */

// const f: (x: number) => number = x => x + 2;
// const g: (x: number) => number = x => x * 3;

/*
the `{}` is actually `U` but the type checker failed to infer it, see the compiled version
See https://stackoverflow.com/questions/36967176/what-is-type
*/
let composeBeforeId: <T, U>(f: (x: T) => U) => (x: T) => {}
    = f => compose(f)(id);

let composeAfterId: <T, U>(f: (x: T) => U) => (x: T) => {}
    = f => compose(id)(f);

// assert(compose(f)(id)(5) === f(5));
// assert(compose(id)(g)(7) === g(7));