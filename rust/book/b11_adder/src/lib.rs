#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
    }

    #[test]
    #[ignore]
    fn another() {
        // -- --test-threads=1  // not run parallel
        // cargo test an    // run all start with an
        // -- --ignored // only run ignored
        panic!("Make this test fail");
    }

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // custom msg
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be <= 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// from b5_rectangle
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello!")
}

struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be >= 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be <= 100, got {}.", value);
        }

        Guess {
            value: value
        }
    }
}