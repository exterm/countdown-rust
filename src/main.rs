use std::io::{self, BufRead};
use std::thread;
use std::sync::mpsc;

mod compute;

fn main() {
  let stdin = io::stdin();
  let mut iterator = stdin.lock().lines();
  let numbers_string = iterator.next().unwrap().unwrap();
  let target = iterator.next().unwrap().unwrap().parse::<u32>().unwrap();

  let numbers = parse_numbers(&numbers_string);

  println!("{:?}", numbers);
  println!("{:?}", target);

  compute::something();

  let (tx, rx) = mpsc::channel();

  let tx1 = mpsc::Sender::clone(&tx);
  thread::spawn(move || {
    compute::transmit(tx1, 1);
  });

  thread::spawn(move || {
    compute::transmit(tx, 2);
  });

  for received in rx {
    println!("Got: {}", received);
  }

  println!("hung up")
}

fn parse_numbers(numbers: &str) -> std::vec::Vec<u32> {
  let elements: Vec<&str> = numbers.trim_matches(|p| p == '[' || p == ']' )
                                   .split(",")
                                   .collect();

    elements.iter().map(|e| e.parse::<u32>().unwrap()).collect()
}
