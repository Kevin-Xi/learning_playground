fn main() {
    loop {
        println!("Hello, world!");
        break;
    }

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    // both safety & no overhead compare to maintain index of
    // array by while
    for element in a.iter() {
        println!("the value is {}", element);
    }

    // for is most commonly used, replace while:
    for count in (1..4).rev() {
        println!("{}!", count);
    }
    println!("LIFTOFF!!!");

    for n in 1..10 {
        println!("fib {} is {}", n, fib(n));
    }
}

fn fib(n: i32) -> i32 {
    let mut cur = 1;
    let mut nxt = 1;
    let mut res = 0;

    for _ in 0..n {
        res = cur;
        cur = nxt;
        nxt = res + nxt;
    }

    res
}
