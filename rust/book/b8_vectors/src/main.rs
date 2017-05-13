fn main() {
    // if no element at first, need declare type for compiler
    let v: Vec<i32> = Vec::new();

    // macro
    let v2 = vec![1, 2, 3];

    // no need to declare
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // scope
    {
        let v = vec![1, 2, 3, 4];
    } // this v and all of its contents dropped

    {
        let mut ins = Vec::new();
        println!("Input numbers: ");
        loop {
            let mut iw = String::new();
            std::io::stdin().read_line(&mut iw).expect("bad input");

            let iv = match iw.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };

            ins.push(iv);
        }
        // will get a runtime panic if length not enough
        // let third: &i32 = &ins[4];
        let third_op: Option<&i32> = ins.get(4);
        if let Some(num) = ins.get(4) {
            println!("fourth is {}", num);
        } else {
            println!("Nothing there");
        }
    }

    //
    {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];  // immutable borrow
        // v.push(6);  // meanwhile, mutable borrow, so not check
        // why check this? (seems first and last is two element)
        // because push maybe re-allocate all vector so that old
        // reference will hang. borrow checker prevent this
    }

    //
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // they *are* the same type
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
