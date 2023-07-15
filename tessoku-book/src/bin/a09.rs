use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize,usize,usize,usize);n]
    }

    let mut dp = vec![vec![0_isize; w + 1]; h + 1];
    for &(a, b, c, d) in abcd.iter() {
        dp[a][b] += 1;
        dp[c + 1][b] -= 1;
        dp[a][d + 1] -= 1;
        dp[c + 1][d + 1] += 1;
    }

    for i in 0..h {
        let mut v = 0;
        let row = (0..w)
            .map(|j| {
                dp[i + 1][j + 1] += dp[i][j + 1];
                v += dp[i + 1][j + 1];
                v
            })
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", row);
    }
}
