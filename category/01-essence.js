'use strict';
exports.__esModule = true;
/* prelude */
var id = function (x) { return x; };
var compose = function (f) { return function (g) { return function (x) { return g(f(x)); }; }; };
/* prelude end */
// const f: (x: number) => number = x => x + 2;
// const g: (x: number) => number = x => x * 3;
var composeBeforeId = function (f) { return compose(f)(id); };
var composeAfterId = function (f) { return compose(id)(f); };

console.log((composeAfterId(x => x + 1)(2)).toString());