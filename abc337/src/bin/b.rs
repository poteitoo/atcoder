use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut cou = 0;
    for i in 1..s.len() {
        if s[i - 1] != s[i] {
            if s[i - 1] == 'A' && s[i] == 'B' {
                cou += 1;
            } else if s[i - 1] == 'A' && s[i] == 'C' {
                cou += 1;
            } else if s[i - 1] == 'B' && s[i] == 'C' {
                cou += 1;
            } else {
                println!("No");
                return;
            }

            if cou > 2 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes")
}
