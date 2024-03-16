use proconio::{input, marker::Chars};
use std::collections::HashSet;

fn swap_chars(s: &mut Vec<char>, i: usize, j: usize) {
    s.swap(i, j);
}

fn main() {
    input! {
        s: Chars,
    }

    let mut s = s;
    let n = s.len();
    let mut ans = HashSet::new();

    for i in 0..n {
        for j in (i + 1)..n {
            swap_chars(&mut s, i, j);
            ans.insert(s.iter().collect::<String>());
            swap_chars(&mut s, i, j); // 元に戻す
        }
    }

    println!("{}", ans.len());
}
