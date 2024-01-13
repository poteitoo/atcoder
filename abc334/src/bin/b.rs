use proconio::input;

fn main() {
    input! {
        a: isize,
        m: isize,
        l: isize,
        r: isize
    }
    let a = a % m;
    let d = r - l;
    println!("{}", a);
    println!("{}", d);
    // println!("{}", if d % m < a { d / m + 1 } else { d / m });
}
