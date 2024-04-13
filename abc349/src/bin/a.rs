use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n-1]
    }

    let mut pos = 0;
    let mut neg = 0;
    for aa in a.iter() {
        if *aa > 0 {
            pos += aa;
        } else if *aa < 0 {
            neg += aa;
        }
    }
    println!("{}", -(neg + pos))
}
