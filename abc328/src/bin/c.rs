use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q]
    }

    let mut dp = vec![0; n];
    for i in 1..n {
        if s[i - 1] == s[i] {
            dp[i] = dp[i - 1] + 1;
        } else {
            dp[i] = dp[i - 1]
        }
    }

    for (l, r) in lr {
        println!("{}", dp[r - 1] - dp[l - 1]);
    }
}
