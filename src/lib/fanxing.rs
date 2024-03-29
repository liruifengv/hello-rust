pub fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
      if item > largest {
          largest = item;
      }
  }

  largest
}
