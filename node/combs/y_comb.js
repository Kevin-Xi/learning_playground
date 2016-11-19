'use strict';
/**(function (f) {
    f(f);
})(function (f) {
    f(f)
});**/

/**((f) => (n) => {if (n) {console.log(n); f(f)(n-1);}})
((f) => (n) => {if (n) {console.log(n); f(f)(n-1);}})
(10);**/

let y = job => time => {
    ((f) => (n) => {if (n) {job(n); f(f)(n-1);}})
    ((f) => (n) => {if (n) {job(n); f(f)(n-1);}})
    (time);
}

let s = 0;
let arr = [1, 2, 3, 4, 5, 6, 7];
let acc = i => s += arr[i];
y(acc)(arr.length-1);
console.log(s);
