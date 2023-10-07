use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut res = vec![0; n];

    for x in 0..n {
        for y in 0..n {
            if x == y {
                continue;
            }
            if s[x][y] == 'o' {
                res[x] += 1;
            } else if s[x][y] == 'x' {
                res[y] += 1;
            }
        }
    }

    let mut ans: Vec<(usize, &i32)> = res.iter().enumerate().collect();
    ans.sort_by(|a, b| b.1.cmp(a.1));
    let ans = ans.iter().map(|(i, _)| i + 1).join(" ");
    print!("{}", ans);
}
