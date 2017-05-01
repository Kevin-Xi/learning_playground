fn main() {
    // ---
    // 1. must ask OS to allocate at runtime
    // 2. must return to OS when done!
        // use GC
        // return by hand - error-prone
        // free automatically when out of scope - Rust, C++ RAII
        // ? is there a lock for every var as overhead?
        // pro with GC(seems similar to ref count)
    let mut s = String::from("hello");  // construct use String, allocate from heap, mutable
    s.push_str(", world!");
    println!("{}", s);

    // ---
    // move
    // know not size, allocate in heap, will not auto copy
    let s1 = String::from("hello");
    let s2 = s1;    // s1 moved into s2(cut & paste)
    // println!("{}", s1);  // s1 is invalid

    // clone
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // size known, stored on stack
    // integers has Copy trait
    // if a type has Drop trait, it can not has Copy trait
    let x = 5;
    let y = x;  // no diff between shallow & deep copy, and will not invalid x
    // feels like '=' is inconsistence
    println!("x = {}, y = {}", x, y);

    // ---
    // call functions to transfer ownership
    // pass value similar to assign value
    let str = String::from("hello");
    takes_ownership(str); // moved
    // println!("str = {}", str);

    let num = 5;
    makes_copy(num);    // copied
    println!("num = {}", num);  // still here

    // ---
    // return to transfer ownership
    let rs1 = gives_ownership();
    let rs2 = String::from("hello");
    let rs3 = takes_and_gives_back(rs2);

    // ---
    // sometime don't want to transfer because need to transfer back useful thing to use it again
    // like call a function to calc the len of str
    // if want use str after call, need let the function transfer
    // str back
    // so use Reference
}   // num, str out of scope. Nothing happened for str since it moved
// rs1 & rs3 drop, nothing to rs2

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // drop some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    // i32 no Drop trait, nothing happened
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
