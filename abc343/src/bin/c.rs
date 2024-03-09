use proconio::input;

fn is_palindrome(s: &str) -> bool {
    s.bytes().eq(s.bytes().rev())
}

fn main() {
    input! {
        n: usize,
    }

    let mut rep = Vec::new();
    for i in 1..n {
        let cubic = i * i * i;
        if n >= cubic {
            rep.push(cubic);
        } else {
            break;
        }
    }
    rep.reverse();

    for r in rep.iter() {
        if is_palindrome(r.to_string().as_str()) {
            println!("{}", r);
            return;
        }
    }
    println!("1")
}
