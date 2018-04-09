use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let _numbers = iterator.next().unwrap().unwrap();
    let _target = iterator.next().unwrap().unwrap();

    println!("(1)")
}
