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
}
