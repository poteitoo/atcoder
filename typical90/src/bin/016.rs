// use num_iter::range_inclusive;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
        c:usize
    }

    // O(10**9)
    // let mut dp = vec![std::usize::MAX - 1; n + 1];
    // dp[0] = 0;

    // let min_step = a.min(b.min(c));

    // for i in range_inclusive(min_step, n) {
    //     if i >= a {
    //         dp[i] = dp[i].min(dp[i - a] + 1);
    //     }
    //     if i >= b {
    //         dp[i] = dp[i].min(dp[i - b] + 1);
    //     }
    //     if i >= c {
    //         dp[i] = dp[i].min(dp[i - c] + 1);
    //     }
    // }
    // println!("{}", dp[n]);

    // O(10**4)
    let mut res = std::usize::MAX - 1;

    for x in 0..10000 {
        for y in (0..(10000 - x)).rev() {
            let ab = a * x + b * y;
            if n >= ab {
                let nab = n - ab;
                if nab % c == 0 {
                    let z = nab / c;
                    res = res.min(x + y + z);
                }
            }
        }
    }
    println!("{}", res);
}
