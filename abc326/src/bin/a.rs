use proconio::input;

fn main() {
    input! {
        x: isize,
        y: isize
    }

    let diff = x - y;
    if -2 <= diff && diff <= 3 {
        println!("Yes")
    } else {
        println!("No")
    }
}
