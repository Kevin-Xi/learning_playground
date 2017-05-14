fn main() {
    // rust only has `str(string slice)` in the core
    // `String` as rust standard lib
    {
        let s = String::new();
    }

    //
    {
        let data = "initial contents";
        let s = data.to_string();
    }
    // equivalent
    {
        let s = String::from("initial contents");
    }

    //
    {
        let mut s = String::from("foo");
        s.push_str("bar");  // takes string slice
        let s2 = String::from("baz");
        s.push_str(&s2);

        // append a char
        s.push('|');
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;  // +(self, s: &str) -> String
        // &s2 is a &String, rust deref coercion this to &str
        // (turn &s2 to &s2[..])
        // s1 will be moved into + and invalid after this line
        // so + not a copy.

        {
            let s1 = String::from("tic");
            let s1 = String::from("tac");
            let s1 = String::from("toe");

            let s = format!("{}-{}-{}", s1, s2, s3);
        }
    }

    {
        let s1 = String::from("hello");
        // let h = s1[0];   // not support, why?
        // String is a wrapper over Vec<u8>
        let len = String::from("Hola").len();   // 4
        let len_u = String::from("Здравствуйте").len();   // 24 = 12 * 2
        // not a valid char itself in index 0. so not impl to prevent
        // misunderstanding

        let hello = "Здравствуйте";
        let s = &hello[0..4];
        // &hello[0..1] will panic at runtime
    }

    {
        let namasute = "नमस्ते";
        for c in namasute.chars() {
            println!("{}", c);
        }
        for b in namasute.bytes() {
            println!("{}", b);
        }
    }
}
