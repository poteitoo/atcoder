use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let ans = (0..n).into_iter().map(|_| "10").join("");
    println!("{}", ans + "1")
}
