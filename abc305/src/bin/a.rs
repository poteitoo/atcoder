use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let a = n / 5;
    let b = n % 5;

    if a == 20 {
        println!("100");
    } else if b > 2 {
        println!("{}", a * 5 + 5);
    } else {
        println!("{}", a * 5);
    }
}
