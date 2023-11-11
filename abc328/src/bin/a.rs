use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: [usize; n],
    }

    let ans: usize = s.iter().filter(|v| **v <= x).sum();
    println!("{}", ans);
}
