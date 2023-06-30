use proconio::input;

fn main() {
    input! {
        n:usize,
        s:[String;n]
    }

    for (i, s1) in s.iter().enumerate() {
        for t in i + 1..n {
            let c = format!("{}{}", s1, s[t]);
            let rc: String = c.chars().rev().collect();
            if c == rc {
                println!("Yes");
                return;
            }
        }
    }
    println!("No")
}
