fn main() {
    // ---
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // let bytes = test_rt(&s); // immutable borrow
    s.clear();  // but word still be 5

    // ---
    let s2 = String::from("hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];

    // ---
    // although a non-intersect slice, will not work
    /*
    let mut s3 = String::from("hello world");
    let m_hello = &mut s3[0..5];
    let m_world = &mut s3[6..11];
    */

    // ---
    // range
    let slice = &s2[..2];
    let slice = &s2[3..];
    let slice = &s2[..];
    let slice = &s2[0..s.len()];

    // ---
    let s3 = String::from("hello world");
    let word = first_word2(&s3);    // immutable borrow
    // s3.clear();  // mutable borrow not work because word still here

    // ---
    // string literals are slices, so it is immutable(&str)
    let s4 = "Hello, world!";
    // often take &str as argument to make fn more general
    let word = first_word3(&s3[..]);
    let word = first_word3(s4);
}

// the result only valid with the s. If s change, the result
// is meaningless
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn test_rt(s: &String) -> &[u8] {
    s.as_bytes()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
