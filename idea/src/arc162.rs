use itertools::Itertools;
use proconio::input;

fn main() {
    input! {t:usize}
    for _ in 0..t {
        input! {n:usize, p:[isize;n]}
        let arr: Vec<isize> = p
            .iter()
            .enumerate()
            .map(|(i, n)| i as isize - *n as isize)
            .filter(|s| *s >= -1)
            .collect_vec();
        // println!("dist: {:?}", arr);
        let dist = min_edit_distance(&arr);
        // println!("dist: {}", dist);

        if dist / 2 == 0 {
            println!("{}", dist);
        } else {
            println!("{}", dist / 2);
        }
    }
}

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
