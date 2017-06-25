// https://hackernoon.com/from-callback-to-future-functor-monad-6c86d9c16cb5

'use strict';

function Future() {
    this.slots = [];
}

Future.prototype.ready = function (slot) {
    if (this.completed) {
        slot(this.value);
    } else {
        this.slots.push(slot);
    }
}

Future.prototype.complete = function (value) {
    if (this.completed) {
        throw new Error('already completed');
    }

    this.completed = true;
    this.value = value;

    this.slots.forEach(slot => slot(value));

    this.alots = null;
}

// (this: Future<a>, (a -> b)) -> Future<b>
// this function made Future a Functor, as
// the Future now can take a normal function
// to apply to the value inside the Future
// and return a Future holding the value after
// applying the normal function
// so as Array etc.
// Actor do acting, Functor do what function do(can compose)
Future.prototype.fmap = function (fn) {
    let f = new Future();
    this.ready(value => f.complete(fn(value)));
    return f;
}

// this: Future<Future<a>> -> Future<a>
Future.prototype.flatten = function () {
    let f = new Future();
    this.ready(insideF => insideF.ready(value => f.complete(value)));
    return f;
}

// (this: Future<a>, a -> Future<b>) -> Future<b>
// if only use fmap, will -> Future<Future<b>>,
// if want b, show wait both the outside and
// the inside Future
// a way to sequencing 2 dependent computations
// this method made Future a Monad: has a way
// to describe a computation as sequence of steps
Future.prototype.flatMap = function (fn) {
    return this.fmap(fn).flatten();
}

// a -> Future<a>
Future.unit = function (value) {
    let f = new Future();
    f.complete(value);
    return f;
}

// (a, Number) -> Future<a>
Future.delay = function (value, ms) {
    let f = new Future();
    setTimeout(() => f.complete(value), ms);
    return f;
}

// lift arity 1
Future.lift1 = function (fn) {
    return fut => fut.fmap(fn);
}

module.exports = Future;