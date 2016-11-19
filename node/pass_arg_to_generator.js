function* logGenerator() {
    console.log(yield);
    console.log(yield);
    console.log(yield);
}

var gen = logGenerator();

gen.next();     // pause at first yield
gen.next('1');  // replace first yield to '1', execute, then pause at second yield
