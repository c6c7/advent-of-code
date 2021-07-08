
pub fn part1(inp: String) {
  println!("Floor {}", get_floor_number(inp.chars(), 0));
}

fn get_floor_number(mut inp: std::str::Chars, start_floor_number: i32) -> i32 {
  match inp.next() {
    None => start_floor_number,
    Some(c) => match c {
      '(' => get_floor_number(inp, start_floor_number + 1),
      ')' => get_floor_number(inp, start_floor_number - 1),
      _ => panic!("Unexpected character '{}'", c)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::year2015::day01::get_floor_number;

  macro_rules! test_cases {
    ( $fn:ident, $([$expected_floor:expr, $input_str:expr]),* $(,)? ) => {
      $(
        assert_eq!($expected_floor, $fn(String::from($input_str).chars(), 0));
      )*
    }
  }

  #[test]
  pub fn empty_input() {
    assert_eq!(0, get_floor_number(String::from("").chars(), 0));
  }

  #[test]
  pub fn simple_cases() {
    test_cases!(get_floor_number,
      [0, ""],
      [1, "("],
      [-1, ")"],
      [0, "()"],
    );
  }

  #[test]
  pub fn website_examples() {
    test_cases!(get_floor_number,
      [0, "(())"],
      [0, "()()"],
      [3, "((("],
      [3, "(()(()("],
      [3, "))((((("],
      [-1, "())"],
      [-1, "))("],
      [-3, ")))"],
      [-3, ")())())"],
    );
  }

  #[test]
  pub fn different_starting_floor() {
    assert_eq!(100, get_floor_number(String::from("").chars(), 100));
    assert_eq!(101, get_floor_number(String::from("(").chars(), 100));
  }
}
