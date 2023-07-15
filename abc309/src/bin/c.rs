use num_iter::range;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n]
    }

    let mut ans = vec![0_usize; 1_000_000_000];
    for (a, b) in ab.iter().cloned() {
        ans[a] += b;
    }
    for i in range(1, 1_000_000_000).rev() {
        ans[i - 1] += ans[i];
        if ans[i - 1] > k {
            println!("{}", i);
            return;
        }
    }
}
