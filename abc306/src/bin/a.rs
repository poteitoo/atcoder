use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        s: Chars
    }

    let mut t = String::new();

    for c in s.iter() {
        t.push(*c);
        t.push(*c);
    }

    println!("{}", t);
}
