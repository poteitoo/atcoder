use num_iter::range_step;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n*7]
    }

    let mut arr: Vec<String> = Vec::new();
    for i in range_step(0, n * 7, 7) {
        let mut sum = 0;
        for j in i..(i + 7) {
            sum += a[j];
        }
        arr.push(sum.to_string());
    }

    println!("{}", arr.join(" "));
}
