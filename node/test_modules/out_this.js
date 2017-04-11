// 1    `this` is `exports` in a module. `exports` is `{}` by default when `module._compile` by node
// exports.a = 1;
// module.exports = this;

// 2    `this` by this kind of function invoke method is `global`
module.exports = (function () {return this;})();
