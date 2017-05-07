// Enum is `one of`, Struct is `each of`
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {  // similar to datatype of SML
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Hello, world!");
    }
}

// ---
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=> {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// ---
fn plus_one(x: Option<i32>) -> Option<i32> {
    // compiler will check if all patterns exhausted
    match x {
        None => None,   // no need to declare type for RHS
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // transfer human effort for null checking to compiler
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    // ---
    let v = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("value in cents is {}", v);

    // ---
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);

    // ---
    let u8_value = 0u8;
    match u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => (),
    }

    // ---
    // match become wordy if only care one case
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("only care three");
    }
    // can write two, no exhaust check
    if let Some(4) = some_u8_value {
        println!("only care four");
    } else {
        println!("not match");
    }
}
