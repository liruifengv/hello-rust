#[allow(dead_code)]

fn function(i: i32) -> i32 {
    i + 1
}

pub fn test_closure() {
    let closure_annotated = |i: i32| -> i32 { i + 1 };

    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));

    let one = || 1;
    println!("closure returning one: {}", one());

    let b: i32 = 2;

    let two_sum = |a| a + b;

    println!("two_sum: {}", two_sum(1));

    fn exec<F: FnOnce()>(f: F) {
        f()
    }

    fn exec1<F: FnMut()>(mut f: F) {
        f()
    }

    fn exec2<F: Fn()>(f: F) {
        f()
    }

    let s = String::new();

    let update_string = || println!("{}", s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);

    // 闭包作为函数的返回值
    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;

        if x > 1 {
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x - num)
        }
    }

    let f = factory(10);

    let answer = f(10);

    println!("{}", answer);
}
