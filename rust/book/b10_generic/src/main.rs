// generics has no runtime overhead

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    // if T only bound to PartialOrd(without Copy),
    // T maybe not a known sized on stack(T can't copied).
    // It seems that `=` will move by default, but `list`
    // is &[T] that can't move, [T] will fix this check.
    // or, use PartialOrd + Clone and use clone() inside,
    // need to allocate in the heap (which is slow) this way.
    // or, return &T instead of T, so no Copy or CLone needed.
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U
}

// <T, U> after impl only go with Point
impl<T, U> Point<T, U> {
    // V, W after mixup only to do with the method
    // if add T, will compile err: T be shadowed
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
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
