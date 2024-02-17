use proconio::input;

const INF: usize = 1_000_000_000_000;
const MAX_VALUE: usize = 100000;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    // x: 価値の合計
    // y: i番目までの品物を使って作れる価値の合計の最小の重さ
    let mut dp = vec![vec![INF; MAX_VALUE + 1]; n + 1];
    dp[0][0] = 0;

    for (i, (ww, vv)) in wv.iter().enumerate() {
        for j in 0..=MAX_VALUE {
            dp[i + 1][j] = if j < *vv {
                dp[i][j]
            } else {
                dp[i][j].min(dp[i][j - vv] + ww)
            }
        }
    }

    // let ans = dp[n]
    //     .iter()
    //     .enumerate()
    //     .rev()
    //     .find(|(_, d)| **d <= w)
    //     .unwrap_or((0, &0))
    //     .0;

    let mut ans = 0;
    for (i, d) in dp[n].iter().enumerate() {
        if *d <= w {
            ans = i;
        }
    }
    println!("{}", ans);
}
