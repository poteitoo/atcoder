use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n]
    }

    let mut dp = vec![0; n];
    for i in 0..n {
        if i == 0 {
            continue;
        }

        let diff1: isize = if i >= 1 {
            (h[i] - h[i - 1]).abs() + dp[i - 1]
        } else {
            isize::MAX
        };
        let diff2 = if i >= 2 {
            (h[i] - h[i - 2]).abs() + dp[i - 2]
        } else {
            isize::MAX
        };

        dp[i] = if diff1 > diff2 { diff2 } else { diff1 };
    }

    println!("{}", dp[n - 1])
}
