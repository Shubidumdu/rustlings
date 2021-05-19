// "//"을 통해 코멘트를 작성할 수 있습니다.
// 이는 컴파일러에 의해 무시됩니다.

// Rust는 `rustc <path>`를 파일을 바이너리로 컴파일링합니다.
// 컴파일 이후 `./01_hello`와 같이 컴파일된 파일(확장자가 없음)을 실행하면 됩니다.

// 아래는 main 함수입니다.
// 컴파일된 바이너리를 호출할 때 여기에서의 Statements들이 실행됩니다.
fn main() {
  // 아래 내용은 콘솔에 텍스트를 출력합니다.
  println!("Hello World!");
  println!("I'm a Rustacean!");

  // 한줄 코멘트
  /*
  여러 줄
  코멘트
  */

  //  `///` 혹은 `//!`는 Doc Comment 라고 하는데, 이에 대해선 추후에 살펴봅시다.

  // 아래처럼 Expression 한 가운데에 코멘트가 껴있어도 문제가 없습니다.
  let x = 5 + /* 90 + */ 5;
  println!("Is `x` 10 or 100? x = {}", x);

  /*
    `std::fmt`에 의해 정의된 매크로들에는 다음과 같은 것들이 있습니다.
    - `format!`: String 타입으로 포맷 텍스트를 작성합니다.
    - `print!` : `format!`과 동일하되, 콘솔에 이를 출력합니다. (`io::stdout`)
    - `println!` : `print!`와 동일하되, 줄이 변경됩니다.
    - `eprint!` : `format!`과 동일하되, standard error로 이를 출력합니다. (`io::stderr`)
    - `eprintln!` : `eprint!`와 동일하되, 줄이 변경됩니다.

    위의 매크로 모두 같은 방식으로 구문을 분석하며, 컴파일 시점에 포맷이 적합한지 체크됩니다.
  */

  // 기본적으로, `{}` 는 뒤의 인수로 대체됩니다.
  println!("{} days", 31);
  // 여러 개의 인수를 가져와서 사용할 수도 있습니다.
  // 별도로 0, 1로 인덱싱을 하지 않더라도, 인수 순서대로 사용합니다.
  println!("{0}, this is {1}. {1}, this is {0}", "Tom", "Jack");
  // 인수에 이름을 붙일 수도 있습니다.
  println!(
    "{subject} {verb} {object}",
    object = "the lazy dog",
    subject = "the quick brown fox",
    verb = "jumps over"
  );
  // `:<trait>`을 붙여 특정 형태로 포맷을 적용하여 사용할 수 있습니다.
  // 여기서는 `:b`가 사용됐는데, 이는 이진법을 적용하여 변환한다는 뜻입니다.
  // 따라서 아래는 인수 2를 받아 10으로 출력한다.
  println!(
    "{} of {:b} people know binary, the other half doesn't",
    1, 2
  );
  // 특정 width만큼 우측으로 띄워 작성할 수 있다.
  // 아래는 `      1`로 출력된다.
  println!("{number:>width$}", number = 1, width = 6);
  // 특정 width만큼 padding을 채워넣을 수도 있다.
  // 아래는 `0000001`로 출력된다.
  println!("{number:>0width$}", number = 1, width = 6);

  // std::fmt은 텍스트 표시를 제어하는 여러 traits들을 포함하며, 아래 두 형태가 중요한 기본 형태다.
  // `fmt::Debug` - `{:?}` 마커를 사용하며, 디버깅 목적으로 텍스트를 포맷한다.
  // `fmt::Display` - `{}` 마커를 사용하며, 좀더 유연하고 이용자 친화적인 형태로 텍스트를 포맷한다.
  // 위에서는 `fmt::Display`를 사용했는데, std 라이브러리는 해당 유형들에 대한 구현을 제공하기 때문이다.
  // fmt::Display trait의 실행은 자동으로 `ToString` trait를 실행시켜 타입을 String으로 변환한다.
}
