use proconio::{input, marker::Chars};

fn main() {
    input! {
        nn: Chars
    }
    for i in 1..nn.len() {
        let a = nn[i - 1].to_digit(10).unwrap();
        let b = nn[i].to_digit(10).unwrap();

        if a <= b {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
