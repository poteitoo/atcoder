use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {mut td:[(i64,i64)]}
    td.sort_unstable();
    td.reverse();

    let mut q = BinaryHeap::new();
    let mut t = 0;
    let mut c = 0;

    loop {
        if q.is_empty() {
            if let Some(&(u, _)) = td.last() {
                t = u
            } else {
                break;
            }
        }
        while let Some(&(u, d)) = td.last() {
            if u != t {
                break;
            }
            td.pop();
            q.push(Reverse(u + d));
        }
        while let Some(Reverse(d)) = q.pop() {
            if d >= t {
                c += 1;
                break;
            }
        }
        t += 1
    }
    println!("{}", c)
}
