'use strict';

const Future = require('./');
const fs = require('fs');

function logF(f) {
    // f.ready(v => console.log(v));
}

function logFDo(f) {
    f.ready(v => console.log(v));
}

logF(Future.unit(42));

logF(Future.delay(43, 1000));

// ---
function readFileF(file, opts) {
    let f = new Future();
    fs.readFile(file, opts, (err, res) => {
        if (err) throw err;
        f.complete(res);
    });
    return f;
}

logF(readFileF('./index.js', { encoding: 'utf8'}));

// ---
// Future<String> -> Future<Number>
// lifted function: promote function applying
// to normal type to function applying on
// boxed type
/*
function lengthF(strF) {
    return strF.fmap(s => s.length);
}
*/

let lengthF = Future.lift1(s => s.length);

// can compose with lifted function, without
// lost in callback
logF(lengthF(readFileF('./index.js', { encoding: 'utf8'})));

// if without lift, should write logic
// inside deeper and deeper
function getLengthAndPrint(strF) {
    strF.ready(v => console.log(v.length));
}

getLengthAndPrint(readFileF('./index.js', { encoding: 'utf8'}));

// ---

// String -> Future<Array<String>>
function readDirF(path) {
    let f = new Future();
    fs.readdir(path, (err, files) => {
        if (err) throw err;
        f.complete(files);
    });
    return f;
}

// (Future<Array<String>>, Array<String> -> Future<String>) -> Future<Future<String>>
// this will log out the inside Future
logF(readDirF('.').fmap(files => readFileF(files[0], { encoding: 'utf8' })));

// wait for the inside Future
readDirF('.').fmap(files => readFileF(files[0], { encoding: 'utf8' })).ready(logF);

// use flatten
logF(readDirF('.').fmap(files => readFileF(files[0], { encoding: 'utf8' })).flatten());

// compose fmap and flatten as flatMap
// readFileF depends on readDirF
// we can chain them, as (Future<a>, a->Future<b>) -> Future<b>, (Future<b>, b->Future<c>) -> Future<c> ...
logFDo(readDirF('.').flatMap(files => readFileF(files[0], { encoding: 'utf8' })));