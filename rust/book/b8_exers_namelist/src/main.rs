// extern crate b8_exers_namelist;
use std::collections::HashMap;

#[derive(Debug)]
struct Namelist {
    depart_staff_map: HashMap<String, Vec<String>>
}

impl Namelist {
    fn add(&mut self, input: &str) {
        let words: Vec<&str> = input.split_whitespace().collect();
        if let (Some(staff), Some(depart)) = (words.get(1), words.get(3)) {
            let lst = self.depart_staff_map.entry(depart.to_owned().to_owned()).or_insert(vec![]);
            (*lst).push(staff.to_owned().to_owned());
            (*lst).sort();
        }
    }

    fn print_by_depart(&self, depart: &str) {
        match self.depart_staff_map.get(depart) {
            Some(lst) => println!("{:?}", lst),
            None => println!("No records for that department!")
        }
    }

    fn print_all(&self) {
        for (depart, lst) in &self.depart_staff_map {
            println!("{}\n\t{:?}\n", depart, lst);
        }
    }
}

fn main() {
    let inputs = [
        "Add Sally to Engineering",
        "Add Amir to Sales",
        "Add Cindy to Sales",
        "Add Kevin to Engineering",
        "Add Bob to Sales",
        "Add David to Engineering",
        "Add Alice to HR",
        "Add Erla to "
    ];

    let mut nl = Namelist { depart_staff_map: HashMap::new() };
    println!("=== Input ===");
    for element in inputs.iter() {
        println!("{}", element);
        nl.add(element);
    }

    println!("=== Output ===");
    nl.print_by_depart("Engineering");
    nl.print_by_depart("Sales");
    nl.print_by_depart("HR");
    nl.print_by_depart("PR");

    nl.print_all();
}
