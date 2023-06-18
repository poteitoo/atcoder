use proconio::input;

fn main() {
    input! {n:usize}
    awful_sum(n);
    sequential_sum(n);
}

fn awful_sum(n: usize) {
    let mut sum: u128 = 0;
    for i in 0..=n {
        sum = sum + i as u128;
    }
    println!("{}", sum);
}

fn sequential_sum(n: usize) {
    let sum = (n + 1) * n / 2;
    println!("{}", sum);
}
