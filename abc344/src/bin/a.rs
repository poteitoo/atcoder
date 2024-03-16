use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut p = 0;
    let mut ans = String::new();
    for c in s {
        if c == '|' {
            p += 1;
        } else {
            if p == 1 {
                continue;
            }
            ans.push(c)
        }
    }
    println!("{}", ans)
}
