use std::collections::HashMap;

fn main() {
    let arr = [1, 6, 3, 3, 5, 8, 7, 2, 7, 7, 4];

    let res = get_mmm(&arr);

    println!("{:?}", res);

    let cons_word = String::from("first");
    println!("{} -> {}", cons_word, to_pig_latin(&cons_word));
    let vow_word = String::from("apple");
    println!("{} -> {}", vow_word, to_pig_latin(&vow_word));
}

fn get_mmm(lst: &[i32]) -> Vec<f64> {
    vec![mean(lst), mid(lst), mode(lst) as f64]
}

fn mean(lst: &[i32]) -> f64 {
    let sum = lst.iter().fold(0, |p, &q| p + q);
    sum as f64 / lst.len() as f64
}

fn mid(lst: &[i32]) -> f64 {
    let mut vec = lst.to_vec();
    vec.sort_by(|x, y| x.cmp(y));

    let len = vec.len();
    if len % 2 == 0 {
        (vec[len/2] + vec[len/2 - 1]) as f64 / 2.0
    } else {
        vec[len/2] as f64
    }
}

fn mode(lst: &[i32]) -> i32 {
    let mut count_map = HashMap::new();
    for num in lst {
        let count = count_map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut max_num = 0;
    let mut max_count = 0;
    for (num, count) in count_map {
        if count > max_count {
            max_num = *num;
            max_count = count;
        }
    }

    max_num
}

fn to_pig_latin(s: &str) -> String {
    match s.chars().nth(0).unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => s.to_owned() + "-hay",
        _ => s[1..].to_owned() + "-" + s[0..1].to_owned().as_str() + "ay"
    }
}
