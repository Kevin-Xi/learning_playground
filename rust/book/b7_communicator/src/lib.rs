// rust look in src/lib by default, everything should declared here
// all are private by default (so compiler know it is unused because
// it can only be used in the file but not used)
// if pub, can be accessed through parent
// if pri, may be accessed only by current module and its child (so will
// warning unused)
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
