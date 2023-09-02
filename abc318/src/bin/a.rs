use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: usize
    }
    if n >= m {
        println!("{}", (n - m) / p + 1);
    } else {
        println!("{}", 0);
    }
}
