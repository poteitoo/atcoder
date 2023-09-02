// バイナリサーチ

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let check = |x: usize| {
        let o = a.iter().fold(0, |p, aa| p + (x / aa));
        o >= k
    };

    let mut left = 0;
    let mut right = 1_000_000;

    while left < right {
        let mid = (left + right) / 2;
        if check(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    println!("{}", left)
}
