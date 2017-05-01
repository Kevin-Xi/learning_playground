#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    // the 1st param of method always self, either &self or &mut self
    // or self
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // fns in Impl block but don't take self are associated functions
    // often used for constructors
    fn square(size: u32) -> Rectangle {
        Rectangle {length: size, width: size}
    }
}

fn main() {
    // ---
    let rect1 = Rectangle { length: 50, width: 30 };

    // with Debug trait, the println! will print it
    println!("rect1 is {:?}", rect1);  // {:#?} for pretty

    println!(
        "The area of the rectangle is {} square pixels.",
        // Rust will auto ref&deref the rect1 as &rect1
        rect1.area()
        // (&rect1).area()  // are the same
    );

    // ---
    let rect2 = Rectangle {length: 40, width: 10};
    let rect3 = Rectangle {length: 45, width: 60};
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // ---
    // :: to indicate scope
    let my_square = Rectangle::square(15);
}
