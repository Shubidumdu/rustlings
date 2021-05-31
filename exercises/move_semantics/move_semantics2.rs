// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM DONE

// 1. `vec0`으로부터 새로운 변수를 복사하여 사용

// fn main() {
//   let vec0 = Vec::new();
//   let vec0_copy = vec0.clone();

//   let mut vec1 = fill_vec(vec0_copy);

//   // Do not change the following line!
//   println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//   vec1.push(88);

//   println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//   let mut vec = vec;

//   vec.push(22);
//   vec.push(44);
//   vec.push(66);

//   vec
// }

// 2.`fill_vec`이 변수에 대한 참조만을 가져오도록 변경

// fn main() {
//   let vec0 = Vec::new();

//   let mut vec1 = fill_vec(&vec0);

//   // Do not change the following line!
//   println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//   vec1.push(88);

//   println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//   let mut vec = vec.clone();

//   vec.push(22);
//   vec.push(44);
//   vec.push(66);

//   vec
// }

// 3. `fill_vec`이 mutable하게 참조를 대여하도록 변경

fn main() {
  let mut vec0 = Vec::new();

  fill_vec(&mut vec0);

  // Do not change the following line!
  println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

  vec0.push(88);

  println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
  let mut vec = vec;

  vec.push(22);
  vec.push(44);
  vec.push(66);
}
