// differences between variables:
// const can be declared in any scope; must annotate type;
// RHS should computed AOT
// todo low level different?
const MAX_POINTS: u32 = 100_000;
// let y = 0;
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;  // this is reassign, can only contain same type
    println!("The value of x is: {}", x);
    let x = "haha"; // this is shadowing, just a new variable
    println!("The value of x is: {}", x);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, _, b) = tup;
    println!("The value of b is: {}", b);
    // let c = tup.3;

    // array: fixed length! on the stack!
    // vector: can grow/shrink size
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let fst = months[0];
    println!("The value of fst is: {}", fst);
    // a warning at build time, a panic at run time!
    // let non = months[12];
}
