use std::sync::mpsc;

pub fn something() -> i32 {
  println!("from module");
  return 0;
}

pub fn transmit(tx: mpsc::Sender<String>, n: i32) {
  let vals = vec![
    format!("{}: hi", n),
    format!("{}: from", n),
    format!("{}: the", n),
    format!("{}: thread", n),
  ];

  for val in vals {
    tx.send(val).unwrap();
  }
}
