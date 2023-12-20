use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let t = s.iter().cloned().rev().collect_vec();
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for (i, ss) in s.iter().cloned().enumerate() {
        for (j, tt) in t.iter().cloned().enumerate() {
            dp[i + 1][j + 1] = if ss == tt {
                dp[i][j] + 1
            } else {
                dp[i][j + 1].max(dp[i + 1][j])
            }
        }
    }
    println!("{}", dp[n][n]);
}
