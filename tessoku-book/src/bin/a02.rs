use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n]
    }

    if a.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
