use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: isize,
        l: isize,
        r: isize,
        a: [isize; n],
    }
    let ans = a.iter().map(|&aa| aa.clamp(l, r)).join(" ");
    println!("{}", ans);
}
