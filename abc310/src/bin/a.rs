use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        d: [usize; n]
    }

    let d_min = d.iter().min().unwrap();
    println!("{}", p.min(d_min + q));
}
