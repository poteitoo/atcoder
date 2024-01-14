use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    for _ in 0..2 {
        for i in 1..n {
            a[i] = a[i].min(a[i - 1] + 1);
        }
        a.reverse();
    }
    let mut ans = 1;
    for (i, &a) in a.iter().enumerate() {
        ans = ans.max(a.min(i + 1).min(n - i));
    }
    println!("{}", ans);
}
