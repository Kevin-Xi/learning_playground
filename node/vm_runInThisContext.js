'use strict';

let same = 
    (function(){return this;})() === require('vm').runInThisContext('this');//(function(){return this;})();

console.log(same);
