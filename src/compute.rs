use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn something() -> i32 {
  println!("from module");
  return 0;
}

pub fn transmit(tx: mpsc::Sender<String>) {
  let vals = vec![
    String::from("hi"),
    String::from("from"),
    String::from("the"),
    String::from("thread"),
  ];

  for val in vals {
    tx.send(val).unwrap();
    thread::sleep(Duration::from_secs(1));
  }
}
