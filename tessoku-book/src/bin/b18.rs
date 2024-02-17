use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    // 動的計画法のテーブルを初期化
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    // テーブルを埋める
    for (i, &num) in a.iter().enumerate() {
        for sum in 0..=s {
            dp[i + 1][sum] = dp[i][sum] || (sum >= num && dp[i][sum - num]);
        }
    }

    // 解が存在しない場合は-1を出力
    if !dp[n][s] {
        println!("-1");
        return;
    }

    // 解を逆順で取り出す
    let mut sum = s;
    let mut ans = Vec::new();
    for i in (0..n).rev() {
        if !dp[i][sum] {
            sum -= a[i];
            ans.push(i + 1)
        }
    }

    // 結果を出力
    println!("{}", ans.len());
    println!("{}", ans.iter().rev().join(" "));
}
