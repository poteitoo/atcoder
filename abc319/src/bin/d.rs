use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::input;

fn can_divide(cumsum: &Vec<i64>, k: usize, mid: i64) -> bool {
    let mut count = 0;
    let mut last = 0;
    for &sum in cumsum.iter() {
        if sum - last > mid {
            last = sum;
            count += 1;
        }
    }
    count < k
}

fn min_sum_partition(cumsum: Vec<i64>, k: usize) -> i64 {
    let mut low: i64 = 0;
    let mut high = *cumsum.last().unwrap_or(&0);
    while low < high {
        let mid = (low + high) / 2;
        if can_divide(&cumsum, k, mid) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    low
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ls: [i64; n]
    }

    let cumls: Vec<i64> = ls.iter().cumsum().collect_vec();
    let ans = min_sum_partition(cumls, m);
    println!("{}", ans)
}
