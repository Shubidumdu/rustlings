//! # 1. Hello World
//!
//! ## 2) Formatted Print
//!
//! `std::fmt`에 의해 정의된 매크로들에는 다음과 같은 것들이 있습니다.
//! - `format!`: String 타입으로 포맷 텍스트를 작성합니다.
//! - `print!` : `format!`과 동일하되, 콘솔에 이를 출력합니다. (`io::stdout`)
//! - `println!` : `print!`와 동일하되, 줄이 변경됩니다.
//! - `eprint!` : `format!`과 동일하되, standard error로 이를 출력합니다. (`io::stderr`)
//! - `eprintln!` : `eprint!`와 동일하되, 줄이 변경됩니다.
//!
//! 위의 매크로 모두 같은 방식으로 구문을 분석하며, 컴파일 시점에 포맷이 적합한지 체크됩니다.
//!
//!
//! 여러 개의 인수를 가져와서 사용할 수도 있습니다. 별도로 0, 1과 같이 인덱싱을 하지 않더라도, 인수의 순서대로 사용합니다.
//! ```rust
//! println!("{0}, this is {1}. {1}, this is {0}", "Tom", "Jack");
//! ```
//!
//! 인수에 이름을 붙일 수도 있습니다.
//!
//! ```rust
//! println!(
//!   "{subject} {verb} {object}",
//!   object = "the lazy dog",
//!   subject = "the quick brown fox",
//!   verb = "jumps over"
//! );
//! ```
//!
//! `:<trait>`를 붙여 특정 형태로 포맷을 적용하여 사용할 수 있습니다.
//! 아래에서는 `:b`가 사용됐는데, 이는 이진법을 적용하여 변환한다는 뜻입니다.
//! 따라서 아래는 인수 2를 받아 10으로 출력합니다.
//!
//! ```rust
//! println!(
//!   "{} of {:b} people know binary, the other half doesn't",
//!   1, 2
//! );
//! ```
//!
//! 특정 width만큼 우측으로 띄워서 출력할 수도 있습니다.
//! 아래는 `      1`으로, 좌측에 6개의 여백이 생깁니다.
//!
//! ```rust
//! println!("{number:>width$}", number = 1, width = 6);
//! ```
//!
//! 특정 width만큼 padding을 채워넣을 수도 있습니다.
//! 아래는 `0000001`로 출력됩니다.
//!
//! ```rust
//! println!("{number:>0width$}", number = 1, width = 6);
//! ```
//!
//! std::fmt은 텍스트 표시를 제어하는 여러 traits들을 포함하며, 아래 두 형태가 중요한 기본 형태다.
//!
//! - `fmt::Debug` - `{:?}` 마커를 사용하며, 디버깅 목적으로 텍스트를 포맷한다.
//! - `fmt::Display` - `{}` 마커를 사용하며, 좀더 유연하고 이용자 친화적인 형태로 텍스트를 포맷한다.
//!
//! 위의 예시에서는 `fmt::Display`를 사용했는데, std 라이브러리는 해당 유형들에 대한 구현을 제공하기 때문이다.
//! fmt::Display trait의 실행은 자동으로 `ToString` trait를 실행시켜 타입을 String으로 변환한다.

/// 기본적으로, format 상에서 `{}`는 추가적으로 받는 인수로 대체됩니다.
pub fn example1() {
  println!("{} days", 31);
}

pub fn main() {
  println!("{} days", 31);
  println!("{0}, this is {1}. {1}, this is {0}", "Tom", "Jack");
  println!(
    "{subject} {verb} {object}",
    object = "the lazy dog",
    subject = "the quick brown fox",
    verb = "jumps over"
  );
  println!(
    "{} of {:b} people know binary, the other half doesn't",
    1, 2
  );
  println!("{number:>width$}", number = 1, width = 6);
  println!("{number:>0width$}", number = 1, width = 6);
}
