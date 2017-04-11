// `console.log` to show it is only loaded once
// if the stuff exported will change is a "pass-by-sharing" issue
module.exports = 1;//{a: (()=>{console.log('export'); return 1;})()};
