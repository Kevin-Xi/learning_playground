// under the package b7_communicator, there are two crates:
// lib.rs -- lib crate, main.rs -- binary crate
// they are also root modules (module in top-level)
// extern go b7_communicator and find the root lib module
extern crate b7_communicator;

fn main() {
    b7_communicator::client::connect();
}
