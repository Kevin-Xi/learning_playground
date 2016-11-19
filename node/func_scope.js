var str = 'global';

function printStr () {
    console.log(str);
}

var aPrintStr = () => console.log(str);

function testScope () {
    var str = 'local';
    printStr();
    aPrintStr();
}

str = 'baz';    // because hoisting, str in function's closure is this

testScope();