// under the package b7_communicator, there are two crates:
// lib.rs -- lib crate, main.rs -- binary crate
// they are also root modules (module in top-level)
// extern go b7_communicator and find the root lib module
extern crate b7_communicator;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// enum is also a scope and can be `use`
use TrafficLight::{Red, Yellow};

fn main() {
    b7_communicator::client::connect();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
