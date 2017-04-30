fn main() {
    println!("Hello, world!");

    let tup = (1, 'h', true);
    another_function(tup);
    // another_function2(tup);

    statement_expression();

    let x = five();
    println!("The value of x is {}", x);
}

// 0. must declare params' types
// 1. cannot directly destructing as params: fn another_function(x: i32, y: char, z: bool)
// 2. if tup in main declared as (i32, ...), will not type-checked
// 3. another_function2 will not type-checked. So rust will use function params to infer, and use the 1st
fn another_function(tup: (i64, char, bool)) { // declare order not matter
    let (x, y, z) = tup;
    println!("Another function args: {} {} {}.", x, y, z);
}

fn another_function2(tup: (i32, char, bool)) {
    let (x, y, z) = tup;
    println!("Another function2 args: {} {} {}.", x, y, z);
}

// statement perform stuff and return nothing, so it can't be bound
// expression eval and return value
// todo why:
// 1. split statement & expression so clear?
// 2. the return style
fn statement_expression() {
    let x = 5;
    let y = {   // block as expression
        let x = 3;
        x + 1   // expression. with `;` it will become a statement and return nothing(return a `()`, not `4`)!
    };

    println!("The value of x, y is: {}, {}", x, y);
}

fn five() -> i32 {
    5   // fn return the final expression
}
