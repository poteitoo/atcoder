use proconio::input;

fn main() {
    input! {
        mut n: usize
    }
    let mut count = 0;
    while n & 1 == 0 {
        n >>= 1;
        count += 1;
    }
    println!("{}", count)
}
