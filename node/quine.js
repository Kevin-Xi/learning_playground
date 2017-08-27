let src = [
    'let src = [',
    '];',
    'console.log(src[0]);',
    'for (var i = 0; i < src.length - 1; i++) console.log(`    \'${src[i]}\',`);',
    'console.log(`    \'${src[src.length - 1]}\'`);',
    'for (var i = 1; i < src.length; i++) console.log(src[i]);'
];
console.log(src[0]);
for (var i = 0; i < src.length - 1; i++) console.log(`    '${src[i]}',`);
console.log(`    '${src[src.length - 1]}'`);
for (var i = 1; i < src.length; i++) console.log(src[i]);