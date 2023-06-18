use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        t:usize
    }

    for _ in 0..t {
        input! {n:usize, p:[isize;n]}

        let mut hs = HashSet::new();
        let mut ans = 0;

        for i in (0..n).rev() {
            if (0..p[i]).all(|x| !hs.contains(&x)) {
                ans += 1;
            }
            hs.insert(p[i]);
        }
        println!("{}", ans);
    }
}
