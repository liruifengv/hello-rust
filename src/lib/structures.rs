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
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: Point,
        bottom_right: Point,
    }

    pub fn show_struct() {
        let name = String::from("Peter");
        let age = 27;
        let peter = Person { name, age };
        // Print debug struct
        println!("{:?}", peter);
    }
}
