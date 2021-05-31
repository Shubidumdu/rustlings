// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

// I AM NOT DONE

fn main() {
  let vec0 = Vec::new();

  let mut vec1 = fill_vec(vec0);

  println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

  vec1.push(88);

  println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// vec의 소유권을 이동시키면서 mutable하게 변경합니다.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
  vec.push(22);
  vec.push(44);
  vec.push(66);

  vec
}
