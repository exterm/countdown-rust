use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();
  let mut iterator = stdin.lock().lines();
  let numbers_string = iterator.next().unwrap().unwrap();
  let target = iterator.next().unwrap().unwrap().parse::<u32>().unwrap();

  let numbers = parse_numbers(&numbers_string);

  println!("{:?}", numbers);
  println!("{:?}", target);

  println!("(1)")
}

fn parse_numbers(numbers: &str) -> std::vec::Vec<u32> {
  let elements: Vec<&str> = numbers.trim_matches(|p| p == '[' || p == ']' )
                                   .split(",")
                                   .collect();

    elements.iter().map(|e| e.parse::<u32>().unwrap()).collect()
}
