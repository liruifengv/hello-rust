#[allow(dead_code)]
mod lib;

use crate::lib::enums;
use crate::lib::front_of_house;
use crate::lib::structures;

#[derive(Debug)]
struct Structure(i32);
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    front_of_house::add_to_waitlist();
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

    front_of_house::test_tuple();
    front_of_house::test_arr();
    structures::show_struct();

    let top_left = structures::Point { x: 1.0, y: 2.0 };
    let bottom_right = structures::Point { x: 2.0, y: 15.0 };
    let _rectangle = structures::Rectangle {
        top_left: top_left,
        bottom_right: bottom_right,
    };
    let res = structures::rect_area(_rectangle);
    println!("res: {:?}", res);

    // 不能复用 top_left 变量因为被移动到_rectangle 里了。给 Point 结构体加上 copy 和 clone 的宏
    // https://doc.rust-lang.org/error-index.html#E0382
    let res2 = structures::square(top_left, 3.0);
    println!("res2: {:?}", res2);

    let pressed = enums::WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = enums::WebEvent::Paste("my text".to_owned());
    let click = enums::WebEvent::Click { x: 20, y: 80 };
    let load = enums::WebEvent::PageLoad;
    let unload = enums::WebEvent::PageUnload;

    enums::inspect(pressed);
    enums::inspect(pasted);
    enums::inspect(click);
    enums::inspect(load);
    enums::inspect(unload);
}
