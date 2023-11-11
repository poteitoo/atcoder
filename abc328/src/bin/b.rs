use proconio::input;

fn is_zorome(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let first_char = s.chars().next().unwrap();
    s.chars().all(|c| c == first_char)
}

fn main() {
    input! {
        n: usize,
        d: [usize; n]
    }

    let mut cou = 0;

    for (i, dd) in d.iter().enumerate() {
        let m = i + 1;
        for ddd in 1..=*dd {
            let chars = format!("{}{}", m, ddd);
            if is_zorome(&chars) {
                cou += 1;
            }
        }
    }
    println!("{}", cou);
}
