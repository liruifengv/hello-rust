mod lib;

use crate::lib::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
    println!("Hello, world!");
}
