use proconio::*;

const INF: isize = 1_000_000_000_000;

fn main() {
    input! {z:[(u32,isize)]}
    let mut d = 0;
    let mut e: isize = -INF;
    for &(x, y) in &z {
        if x == 1 {
            e = e.max(d + y);
        } else {
            d = d.max(d.max(e) + y);
        }
    }
    println!("{}", d.max(e));
}
