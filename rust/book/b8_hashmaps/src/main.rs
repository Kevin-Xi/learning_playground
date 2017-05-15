// less used, not prelude, no macro support
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // all k same type, all v same type
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        // need declare, collect can return many kind of collections
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    }

    {   // ownership
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // owned value String has been moved into map, invalid here
        // type with Copy trait will copied into map

        // if pass ref, value that ref to must cover all lifetime
        // of the map
    }

    {
        let team_name = String::from("Blue");
        let score = scores.get(&team_name); // Some(10)

        for (key, value) in &scores {   // arbitrary order
                                        // borrowed by score, cannot use `for ... in scores`, that will move scores
            println!("{}: {}", key, value);
        }
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);    // replace
        println!("{:?}", scores);

        // insert only has no value
        // entry -> Enum Entry
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);
    }

    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);   // -> &mut V
            *count += 1;    // deref to get value
        }

        println!("{:?}", map);
    }
}
