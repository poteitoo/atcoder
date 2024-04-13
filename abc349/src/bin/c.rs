use proconio::{input, marker::Chars};

fn is_abc_sequence(s: &Vec<char>, a: char, b: char, c: char) -> bool {
    let mut i = 0;
    for ch in s.iter() {
        if *ch == a && i == 0 {
            i += 1;
        } else if *ch == b && i == 1 {
            i += 1;
        } else if *ch == c && i == 2 {
            return true;
        }
    }
    false
}

fn main() {
    input! {
        mut s: Chars,
        t: Chars
    }

    s.push('x');
    let s = s.iter().map(|c| c.to_ascii_uppercase()).collect();

    if is_abc_sequence(&s, t[0], t[1], t[2]) {
        println!("Yes");
    } else {
        println!("No");
    }
}
