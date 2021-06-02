// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

// I AM DONE

fn main() {
  let answer = current_favorite_color();
  println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
  // 1. `to_string()` 메서드를 사용하는 방법
  // "blue".to_string()

  // 2. `String::from` 트레이트를 사용하는 방법
  String::from("blue")
}
