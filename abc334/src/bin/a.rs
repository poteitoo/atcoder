use proconio::input;

fn main() {
    input! {
        bg: (usize, usize)
    }
    println!("{}", if bg.0 > bg.1 { "Bat" } else { "Glove" })
}
