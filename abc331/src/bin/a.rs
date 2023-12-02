use proconio::input;

fn main() {
    input! {
        m: usize,
        d: usize,
        mut yy: usize,
        mut mm: usize,
        mut dd: usize,
    }
    dd += 1;
    if dd > d {
        dd = 1;
        mm += 1;
    }
    if mm > m {
        mm = 1;
        yy += 1;
    }
    println!("{} {} {}", yy, mm, dd)
}
