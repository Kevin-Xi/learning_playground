// 1. constructor
// it is actually just a normal function, but we define it in our mind
// as a constructor, we will call it by `new`, not other way.
function Animal(id) {
    this.cls = 'Animal';
    this.id = id;
}

Animal.prototype.showId = function () {
    console.log(this.id);
}

Animal.prototype.showCls = function () {
    console.log(this.cls);
}

// 2. `Animal` is a normal function so it owns properties [ 'length', 'name', 'arguments', 'caller', 'prototype' ]
console.log(Object.getOwnPropertyNames(Animal));
// `this.cls` in function body is irrelevant here, only `Animal.cls = ...` will set the value
console.log(Animal.cls);
Animal.cls = 'set mannually';
console.log(Animal.cls);

// 3. we want `Cat` to be subclass(make use of all properties of superclass) of `Animal`
function Cat(id) {
    // to get that, we call superclass' constructor on this(support to be a `Cat`)
    Animal.call(this, id);
    // at first `cls` will be `Animal` which set in Animal
    console.log(this.cls);
    this.cls = 'Cat';
    console.log(this.cls);
}

// and then 
Cat.prototype = Object.create(Animal.prototype);
Cat.prototype.constructor = Cat;

let cat = new Cat(1);
cat.showId();
cat.showCls();
