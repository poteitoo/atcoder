use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String
    }
    let regex = Regex::new(r"^<+=+>$").unwrap();
    if regex.is_match(&s) {
        println!("Yes")
    } else {
        println!("No")
    }
}
