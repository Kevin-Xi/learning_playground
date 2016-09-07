// http://stackoverflow.com/questions/6398196/node-js-detect-if-called-through-require-or-directly-by-command-line
var a = 1;

if (require.main === module) {
    // run directly
    console.log(a);
} else {
    // via require
    module.exports = a;
}
