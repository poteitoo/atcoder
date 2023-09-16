use proconio::input;

fn longest_palindrome(s: &str) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }

    let mut dp = vec![vec![false; n]; n];
    let mut left = 0;
    let mut right = 0;

    for i in (0..n).rev() {
        dp[i][i] = true;
        for j in i + 1..n {
            if s.as_bytes()[i] == s.as_bytes()[j] {
                if j - i == 1 {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = dp[i + 1][j - 1];
                }
                if dp[i][j] && j - i > right - left {
                    left = i;
                    right = j;
                }
            }
        }
    }

    right - left
}

fn main() {
    input! {
        s: String
    }
    let ans = longest_palindrome(s.as_str());
    println!("{}", ans + 1)
}
