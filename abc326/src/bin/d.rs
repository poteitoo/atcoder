// use proconio::{input, marker::Chars};

use proconio::input;

fn main() {
    input! {
        n: usize,
        // r: Chars,
        // c: Chars
    }

    let mut cands = vec!['A', 'B', 'C'];
    for _ in 0..n - 3 {
        cands.push('.');
    }
    println!("{:?}", cands);
}
