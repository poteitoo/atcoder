use proconio::input;

fn main() {
    input! {
        n: usize,
        wx: [(usize, usize); n]
    }

    let mut arr = vec![0; 24];
    for (w, x) in wx {
        for i in 0..9 {
            arr[(x + i) % 24] += w;
        }
    }
    let ans = arr.iter().max().unwrap();
    println!("{}", ans);
}
