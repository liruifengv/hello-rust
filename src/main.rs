#[allow(dead_code)]
mod lib;

use crate::lib::front_of_house::hosting;
use crate::lib::structures::aaa;

#[derive(Debug)]
struct Structure(i32);
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    hosting::add_to_waitlist();
    println!("Hello, world!");
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("Base 10 repr:               {}", 69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);
    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);
    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}");
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Variables can be type annotated.
    // 类型推断
    let logical: bool = true;
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    // 变量可以覆盖
    let mutable = true;

    hosting::test_tuple();
    hosting::test_arr();
    aaa::show_struct();
}
