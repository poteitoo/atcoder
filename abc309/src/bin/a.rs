use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }
    let a = a - 1;
    let b = b - 1;

    if a / 3 == b / 3 && a % 3 == b % 3 - 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
