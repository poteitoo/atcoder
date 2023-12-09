use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
        p: [usize; n-1]
    }

    println!("{:?}", p)
}
