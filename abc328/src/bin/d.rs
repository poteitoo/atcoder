use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s: Chars
    }

    let mut ans = String::new();
    for ss in s {
        ans.push(ss);
        if ans.ends_with("ABC") {
            ans.pop();
            ans.pop();
            ans.pop();
        }
    }
    print!("{}", ans)
}
