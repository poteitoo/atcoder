use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    }
    let l = n.lcm(&m);
    let mut ok = 10usize.pow(18) * 3;
    let mut ng = 0;

    while ng + 1 < ok {
        let x = (ok + ng) / 2;
        let y = x / n + x / m - x / l * 2;
        if y < k {
            ng = x
        } else {
            ok = x
        }
    }
    println!("{}", ok)
}
