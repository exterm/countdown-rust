use std::sync::mpsc;

use im;


pub fn sublists<T>(list: im::Vector<T>) -> im::Vector<im::Vector<T>> where T: Clone {
  let opt_head: Option<&T> = list.head();
  match opt_head {
    None => vector![vector![]],
    Some(&head) => {
      let tail = list.skip(1);
      let rest = sublists(tail);
      rest.append(rest.into_iter().map(|e| vector![head].append(e)).collect());
      vector![vector![]]
    }
  }
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

// -spec subs([A]) -> [[A]].
// subs([]) -> [[]];
// subs([H|T]) -> Rest = subs(T),
//                Rest ++ lists:map(fun(L) -> [H|L] end, Rest).

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_sublists() {
    let empty_listlist: im::Vector<im::Vector<i32>> = vector![vector![]];
    assert_eq!(sublists::<i32>(vector![]), empty_listlist);

    let simple_list = vector![1,2];
    let simple_listlist = vector![vector![1,2],vector![1],vector![2]];
    assert_eq!(sublists(simple_list), simple_listlist);
  }
}
