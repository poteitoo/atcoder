use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize
    }

    let mut dp = vec![1_000_000; 1000];
    dp[0] = 0;
    for i in 0..=n {
        dp[i + 6] = dp[i + 6].min(dp[i] + s);
        dp[i + 8] = dp[i + 8].min(dp[i] + m);
        dp[i + 12] = dp[i + 12].min(dp[i] + l);
    }
    let ans = dp[n..].iter().min().unwrap();
    println!("{}", ans);
}
