extern crate b11_adder;

mod common;

#[test]
fn it_adds_two() {
    // --test <name> to only run <name>
    common::setup();
    assert_eq!(4, b11_adder::add_two(2));
}