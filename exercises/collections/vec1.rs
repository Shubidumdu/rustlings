// vec1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute the command `rustlings hint vec1` if you need hints.

// I AM DONE

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
  let a = [10, 20, 30, 40]; // a plain array
                            // TODO: declare your vector here with the macro for vectors

  // 1. Vec::new()로 새 벡터를 만든 후 `push` 메서드로 채워넣는 방법
  let mut v: Vec<i32> = Vec::new();
  for _a in a.iter() {
    v.push(*_a)
  }

  // 2. `vec![]`매크로를 사용하는 방법
  let v = vec![10, 20, 30, 40];

  // 3. `to_vec` 메서드를 사용하는 방법
  let v = a.to_vec();

  (a, v)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_array_and_vec_similarity() {
    let (a, v) = array_and_vec();
    assert_eq!(a, v[..]);
  }
}
