use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n]
    }

    let mut dp = vec![0_isize; d + 1];
    for (l, r) in lr.iter().cloned() {
        dp[l - 1] += 1;
        dp[r] -= 1;
    }

    for i in 0..d {
        dp[i + 1] += dp[i];
        println!("{}", dp[i]);
    }
}
