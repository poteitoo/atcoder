use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }

    let mut ans = vec![0; n + 1];
    let mut x = 0;
    let mut i = 0;
    for a in a {
        ans[a] += 1;
        if ans[a] > x || ans[a] == x && a < i {
            x = ans[a];
            i = a
        }
        println!("{}", i)
    }
}
