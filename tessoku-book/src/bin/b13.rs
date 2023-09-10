use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let a = a.iter().cumsum::<usize>().collect_vec();
    println!("{:?}", a);

    let (mut l, mut ans) = (0, 0);
    for r in 0..n - 1 {
        while l <= r && a[r + 1] - a[l] > k {
            l += 1;
        }
        ans += r - l + 1;
    }
    println!("{}", ans)
}
