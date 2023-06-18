fn min_edit_distance(a: &[i32], b: &[i32]) -> usize {
    let m = a.len();
    let n = b.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }

    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1])) + 1;
            }
        }
    }

    dp[m][n]
}

fn main() {
    let a = vec![1, 2, 3, 4];
    let b = vec![1, 2, 3, 4];

    // let a = vec![
    //     1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    // ];
    // let b = vec![
    //     13, 2, 7, 1, 5, 9, 3, 4, 12, 10, 15, 6, 8, 14, 20, 16, 19, 18, 11, 17,
    // ];

    let distance = min_edit_distance(&a, &b);
    if distance == 0 {
        println!("{}", a.len());
    } else {
        println!("{}", distance / 2);
    }

    println!("Minimum edit distance: {}", distance);
}
