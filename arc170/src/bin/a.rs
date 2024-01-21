use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    }
    // 動的計画法のテーブルを初期化
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                // 文字が一致する場合、前の値を継承
                dp[i][j] += dp[i - 1][j - 1];
            } else if s[i - 1] == 'A' {
                // 文字が一致しない場合、最小の操作回数を計算
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]);
            }
        }
    }
    for d in dp.iter() {
        println!("{:?}", d);
    }
}
