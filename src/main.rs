mod closures;
#[allow(dead_code)]
mod lib;

use crate::lib::enums;
use crate::lib::front_of_house;
use crate::lib::structures;

fn main() {
    front_of_house::hello_world();

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

    closures::test_closure();

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);

    let x = 5;
    // 填写空白处
    let p = &x;
 
    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
}
