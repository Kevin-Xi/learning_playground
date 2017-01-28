use std::io;    // Its `import` and namespace

fn main() {
    println!("Guess the number!");  // `!` means macro

    println!("Please input your guess.");

    let mut guess = String::new();  // default immutable

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess); // String template
}
