use proconio::{fastout, input};
macro_rules! chmax {($base:expr,$($cmps:expr),+$(,)*)=>{{let cmp_max=max!($($cmps),+);if$base<cmp_max{$base=cmp_max;true}else{false}}};}
macro_rules! max {($a:expr$(,)*)=>{{$a}};($a:expr,$b:expr$(,)*)=>{{std::cmp::max($a,$b)}};($a:expr,$($rest:expr),+$(,)*)=>{{std::cmp::max($a,max!($($rest),+))}};}

#[fastout]
fn main() {
    input! {
        n: usize,
        b: [(usize,usize); n]
    };
    let mut dp = vec![0; n];
    for r in (0..n).rev() {
        let mut nx = dp.clone();
        for l in 0..=r {
            let mut lp = 0;
            if l > 0 {
                lp = nx[l - 1];
                let (p, a) = b[l - 1];
                if l < p && p - 1 <= r {
                    lp += a;
                }
            }
            let mut rp = 0;
            if r < n - 1 {
                rp = dp[l];
                let (p, a) = b[r + 1];
                if l < p && p - 1 <= r {
                    rp += a;
                }
            }
            chmax!(nx[l], lp, rp);
        }
        dp = nx;
    }
    println!("{}", dp.iter().max().unwrap());
}
