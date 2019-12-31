extern crate backtrace;

use backtrace::Backtrace;

fn main() {
    foo();
}

fn foo() {
    let bt = Backtrace::new();
    println!("{:?}", bt);
}
