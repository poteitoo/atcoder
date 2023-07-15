use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(Usize1, Usize1); d],
    }
    let mut max_l = vec![0; n + 1];
    let mut max_r = vec![0; n + 1];

    for i in 0..n {
        let rev_i = n - i - 1;
        max_l[i + 1] = max_l[i].max(a[i]);
        max_r[rev_i] = max_r[rev_i + 1].max(a[rev_i]);
    }
    for (l, r) in lr {
        println!("{}", max_l[l].max(max_r[r + 1]));
    }
}
