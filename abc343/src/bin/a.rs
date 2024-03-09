use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let ab = a + b;
    for n in 0..10 {
        if ab != n {
            println!("{}", n);
            return;
        }
    }
}
