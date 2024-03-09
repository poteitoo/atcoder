use itertools::Itertools;
use proconio::{input, input_interactive};

fn main() {
    input! {
        n: usize
    }

    for _ in 0..n {
        input_interactive!(a: [usize; n]);
        let mut ans = Vec::new();
        for (i, aa) in a.iter().enumerate() {
            if *aa == 1 {
                ans.push(i + 1);
            }
        }
        let ans = ans.iter().map(|x| x.to_string()).collect_vec().join(" ");

        println!("{}", ans);
    }
}
