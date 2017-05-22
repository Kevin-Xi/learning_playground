#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

// <T, U> after impl only go with Point
impl<T, U> Point<T, U> {
    // V, W after mixup only to do with the method
    // if add T, will compile err: T be shadowed
    fn mixup<V, W, T>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
