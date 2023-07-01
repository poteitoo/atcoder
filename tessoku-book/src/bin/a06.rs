use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q]
    }
    let mut aa = vec![0_usize; n + 1];
    for i in 0..n {
        aa[i + 1] = aa[i] + a[i];
    }

    for (l, r) in lr.iter() {
        let al = aa[*l - 1];
        let ar = aa[*r];
        println!("{}", ar - al);
    }
}
