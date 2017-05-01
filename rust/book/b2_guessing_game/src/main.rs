extern crate rand;

use std::io;    // Its `import` and namespace
use std::cmp::Ordering;
use rand::Rng;  // use `cargo doc --open` to check docs of whole project

fn main() {
    println!("Guess the number!");  // `!` means macro

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // default immutable

        io::stdin().read_line(&mut guess)   // return a value of type io:Result
            .expect("Failed to read line"); // Result often has a method expect()
                                            // if Err will cause crash
                                            // if Ok will take the value and return

        // guess is shadowed
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); // String template

        match guess.cmp(&secret_number) {   // pattern matching work along with type Ordering, which is an Enum
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
