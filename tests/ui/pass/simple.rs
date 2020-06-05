#[macro_use]
extern crate log_macros;

fn main() {
    call("hello", 42);
}

fn call(string: &str, number: u64) {
    log!("This message has a string like {string} and a number like {number}!");
}
