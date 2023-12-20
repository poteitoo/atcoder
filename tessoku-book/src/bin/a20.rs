use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let mut dp = vec![vec![0; s.len() + 1]; s.len() + 1];

    for (i, ss) in s.iter().cloned().enumerate() {
        for (j, tt) in t.iter().cloned().enumerate() {
            if ss == tt {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }
    println!("{}", dp[s.len()][t.len()]);
}
