use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n]
    }

    let mut ans = Vec::new();
    for (i, (a, b)) in ab.iter().cloned().enumerate() {
        // let prob = b / a;
        ans.push((a, b, i + 1));
        // ans.push((prob, i + 1));
        // if a < b {
        //     let prob = b * (1.0 / a);
        //     ans.push((prob, i + 1));
        // } else {
        //     let prob = -a * (1.0 / b);
        //     ans.push((prob, i + 1));
        // }
    }
    // let mut ans = ab.clone();
    ans.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let ans = ans.iter().map(|(_, _, v)| *v).join(" ");
    println!("{}", ans);
}
