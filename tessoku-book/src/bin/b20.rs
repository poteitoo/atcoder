use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 0..dp.len() {
        dp[i][0] = i;
    }
    for i in 0..dp[0].len() {
        dp[0][i] = i;
    }

    for (i, ss) in s.iter().enumerate() {
        for (j, tt) in t.iter().enumerate() {
            dp[i + 1][j + 1] = if ss == tt {
                dp[i][j]
            } else {
                dp[i][j].min(dp[i + 1][j]).min(dp[i][j + 1]) + 1
            };
        }
    }
    println!("{}", dp[s.len()][t.len()]);
}
