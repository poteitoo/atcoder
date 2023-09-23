use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        a: [isize; n-1]
    }
    let n = n - 1;
    let mut a = a;
    a.sort_unstable();
    let c = x - a[1..n - 1].into_iter().sum::<isize>();

    if c > a[n - 1] {
        println!("-1");
        return;
    }
    if c <= a[0] {
        println!("0");
        return;
    }
    println!("{}", c);
}
