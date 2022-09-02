pub mod hosting {
    use std::fmt;
    use std::mem;
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{},{},{},{}", self.0, self.1, self.2, self.3)
        }
    }

    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // `let` can be used to bind the members of a tuple to variables
        let (integer, boolean) = pair;

        (boolean, integer)
    }
    pub fn add_to_waitlist() {
        println!("add_to_waitlist!");
    }

    pub fn test_tuple() {
        let long_tuple = (
            1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
        );
        // 使用 .index 获取元组里的值
        println!("long tuple first value: {}", long_tuple.0);
        println!("long tuple second value: {}", long_tuple.1);

        // Tuples can be tuple members 多维元组
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

        // Tuples are printable ，超过 12 个元素的元组不能被打印
        println!("tuple of tuples: {:?}", tuple_of_tuples);

        let pair = (1, true);
        println!("pair is {:?}", pair);
        println!("the reversed pair is {:?}", reverse(pair));
        // 创建只有一个元素的元组，需要加逗号
        println!("one element tuple: {:?}", (5u32,));
        println!("just an integer: {:?}", (5u32));
        // 解构元组
        let tuple = (1, "hello", 4.5, true);

        let (a, b, c, d) = tuple;
        println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{}", matrix);
    }

    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    pub fn test_arr() {
        let xs: [i32; 5] = [1, 2, 3, 4, 5];
        println!("array occupies {} bytes", mem::size_of_val(&xs));

        println!("borrow the whole array as a slice");
        analyze_slice(&xs);

        for i in 0..xs.len() + 1 { // OOPS, one element too far
          match xs.get(i) {
              Some(xval) => println!("{}: {}", i, xval),
              None => println!("Slow down! {} is too far!", i),
          }
      }
  
    }
}
