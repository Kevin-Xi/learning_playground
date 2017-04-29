// node --harmony tail_recur.js

console.log((function () {
    'use strict';
    return (function f (n) {
        if (n < 0) return 'foo';
        return f(n-1);
    }(1e6)) === 'foo';
})());
