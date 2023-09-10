use itertools::Itertools;
use num_iter::range_inclusive;
use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let divisors = range_inclusive(1, 9).filter(|x| n % x == 0).collect_vec();
    let num_divisors = divisors.len();

    let mut ans = Vec::new();

    for i in 0..=n {
        for (index, j) in divisors.iter().enumerate() {
            if i * j % n == 0 {
                ans.push(j.to_string());
                break;
            } else if index == num_divisors - 1 {
                ans.push("-".to_string());
            }
        }
    }
    println!("{}", ans.join(""));
}
