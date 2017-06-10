use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str   // if hold ref, need lifetime anno
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // one of the param is self, so all return
    // lifetime is same to self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    // --- lifetime prevent dangling ref
    /*
    let r;

    {   // x not live long enough
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
    */

    // ---
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("Then longest string is {}", result);

    // --- how borrow checker helps after adding lifetime
    // annotations
    /*
    let s1 = String::from("long string is long");
    let result;

    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
    }
    println!("Then longest string is {}", result);
    */

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {part: first_sentence};

    // entire duration of the program
    let s: &'static str = "I have a static lifetime.";
}

// compiler don't know if the result is ref to str1 or str2,
// so the borrow checker don't know enough info to check
// anything here. We programmers need to express our intentions
// mannually, to let borrow checker help us
// fn longest(str1: &str, str2: &str) -> &str {
// since scope is always nested, will be the shortest of str1 and str2
// need to make the lifetime of result relative to at least
// one of the args
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}