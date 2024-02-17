use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for (i, (ww, vv)) in wv.iter().enumerate() {
        for j in 0..=w {
            dp[i + 1][j] = if j < *ww {
                dp[i][j]
            } else {
                dp[i][j].max(dp[i][j - ww] + vv)
            }
        }
    }

    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
