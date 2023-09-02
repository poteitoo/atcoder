use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: usize,
        f: [usize; n]
    }

    let mut f = f;
    f.sort_unstable_by(|a, b| b.cmp(a));

    let ans = f
        .chunks(d)
        .map(|chunk| {
            let sum: usize = chunk.iter().sum();
            if sum > p {
                p
            } else {
                sum
            }
        })
        .collect_vec();

    println!("{}", ans.iter().sum::<usize>());
}
