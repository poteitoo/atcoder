use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut abc: HashSet<char> = HashSet::new();
    for (i, c) in s.iter().cloned().enumerate() {
        if abc.len() == 3 {
            println!("{}", i);
            return;
        }
        abc.insert(c);
    }
    println!("{}", n);
}
