use itertools::Itertools;

fn min_edit_distance(arr: &Vec<isize>) -> usize {
    let n = arr.len();
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 0..=n {
        dp[i][0] = i;
        dp[0][i] = i;
    }

    for i in 1..=n {
        for j in 1..=n {
            if arr[i - 1] == j as isize {
                dp[i][j] = dp[i - 1][j];
            } else {
                let insert_cost = dp[i][j - 1] + 1;
                let delete_cost = dp[i - 1][j] + 1;
                let replace_cost = dp[i - 1][j - 1] + 1;
                dp[i][j] = insert_cost.min(delete_cost.min(replace_cost));
            }
        }
    }

    dp[n][n]
}

fn main() {
    let p = vec![
        13, 2, 7, 1, 5, 9, 3, 4, 12, 10, 15, 6, 8, 14, 20, 16, 19, 18, 11, 17,
    ];

    let mut arr: Vec<isize> = p
        .iter()
        .enumerate()
        .map(|(i, n)| i as isize - *n as isize + 1)
        .collect_vec();

    let tries = min_edit_distance(&arr);

    arr.sort();

    println!("Number of tries: {}", tries);
    println!("Sorted numbers: {:?}", arr);
}
