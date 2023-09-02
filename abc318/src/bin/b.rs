use std::collections::HashSet;

use num_iter::range;
use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize);n]
    }
    let mut set = HashSet::new();

    for (a, b, c, d) in abcd.clone() {
        for x in range(a, b) {
            for y in range(c, d) {
                set.insert((x, y));
            }
        }
    }

    println!("{:?}", set.len());
}
