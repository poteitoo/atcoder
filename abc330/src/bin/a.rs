use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    }
    let mut cou = 0;
    for aa in a.iter() {
        if *aa >= l {
            cou += 1;
        }
    }
    println!("{}", cou);
}
