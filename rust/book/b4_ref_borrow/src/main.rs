fn main() {
    let s1 = String::from("hello");
    // s -> s1 -> heap data, no ownership transfer
    // borrowing
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // change(&s1); // reference is immutable by default

    // ---
    let mut s2 = String::from("hello");
    let a_r = &mut s2;
    change(a_r);
    println!("{}", a_r);    // so &mut is Copy itself
    // seems not: https://www.reddit.com/r/rust/comments/46qwjv/why_can_i_use_an_mut_reference_twice/

    // ---
    // let other_r = &mut s2;   // can only have one ref at a time
                                // a_r are still here so cannot has
                                // anything else &mut to s2
    // for preventing data race at **compile time**
    // see http://rust-lang.github.io/book/second-edition/ch04-02-references-and-borrowing.html#mutable-references
    // ? this way is good for? any cost?

    // ---
    // also cannot combining & and &mut, for same reason
    // let immu_r = &s2;
    // only multiple immu & is ok

    // ---
    // compiler will prevent create dangle ref
    // let ref_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
fn change(s: &String) {
    s.push_str(", world");
}
*/

fn change(s: &mut String) {
    s.push_str(", world");
}

/*
fn ref_to_nothing() -> &String {
    let s = String::from("hello");

    &s
}   // s gone, compiler prevent return &s
    // just return a String s to transfer ownership
*/
