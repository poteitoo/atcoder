use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    print!("{}", s[0]);
    for i in 1..s.len() {
        print!(" {}", s[i]);
    }
}
