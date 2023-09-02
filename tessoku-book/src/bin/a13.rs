// しゃくとり法

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut r = vec![0; n];

    for i in 0..n - 1 {
        if i == 0 {
            r[i] = 0;
        } else {
            r[i] = r[i - 1];
        }

        while r[i] < n && a[r[i]] - a[i] <= k {
            r[i] += 1;
        }
    }

    let ans: usize = r.iter().enumerate().fold(0, |p, (i, x)| p + *x - i);
    println!("{}", ans);
}
