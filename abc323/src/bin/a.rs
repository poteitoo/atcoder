use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    for (i, n) in s.iter().enumerate() {
        if (i + 1) % 2 == 0 {
            if *n == '1' {
                return println!("No");
            }
        }
    }
    println!("Yes");
}
