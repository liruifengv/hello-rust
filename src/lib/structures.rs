pub mod aaa {
    #![allow(dead_code)]

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // A unit struct
    struct Unit;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    #[derive(Debug, Copy, Clone)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }

    // Structs can be reused as fields of another struct
    #[derive(Debug)]
    pub struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        pub top_left: Point,
        pub bottom_right: Point,
    }

    pub fn show_struct() {
        let name = String::from("Peter");
        let age = 27;
        let peter = Person { name, age };
        // Print debug struct
        println!("{:?}", peter);

        let point: Point = Point { x: 10.3, y: 0.4 };
        println!("point coordinates: ({}, {})", point.x, point.y);

        // ... struct update 语法
        let bottom_right = Point { x: 5.2, ..point };
        println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
        // 解构赋值
        let Point {
            x: left_edge,
            y: top_edge,
        } = point;

        // 结构体的实现可以使用结构体
        let _rectangle = Rectangle {
            top_left: Point {
                x: left_edge,
                y: top_edge,
            },
            bottom_right: bottom_right,
        };

        // 定义一个 unit 结构体
        let _unit = Unit;

        // 定义一个元组结构体
        let pair = Pair(1, 0.1);

        // 获取元组结构体的字段
        println!("pair contains {:?} and {:?}", pair.0, pair.1);
        // 解构元组
        let Pair(integer, decimal) = pair;

        println!("pair contains {:?} and {:?}", integer, decimal);
    }

    pub fn rect_area(_rectangle: Rectangle) -> f32 {
        let Point {
            x: left_edge,
            y: top_edge,
        } = _rectangle.top_left;

        let Point {
            x: right_edge,
            y: bottom_edge,
        } = _rectangle.bottom_right;

        (right_edge - left_edge) * (bottom_edge - top_edge)
    }

    pub fn square(point: Point, number: f32) -> Rectangle {
        let Point { x, y } = point;
        Rectangle {
            top_left: point,
            bottom_right: Point {
                x: x + number,
                y: y + number,
            },
        }
    }
}
