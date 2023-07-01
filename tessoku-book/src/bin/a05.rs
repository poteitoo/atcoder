use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }
    let mut ans = 0_usize;

    for i in 1..=n {
        for j in 1..=n {
            if i + j >= k {
                break;
            }
            if i + j + n >= k {
                ans += 1
            }
        }
    }
    println!("{}", ans)
}
