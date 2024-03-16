use proconio::input;

fn main() {
    input! {
        x: i128
    }
    let mut b = x / 10;
    if x % 10 > 0 {
        b += 1;
    }
    println!("{}", b);
}
