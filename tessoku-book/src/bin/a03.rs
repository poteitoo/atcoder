use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
        q: [usize; n],
    }
    for pp in p.iter() {
        let sub = k as isize - *pp as isize;
        if sub < 0 {
            continue;
        }
        let sub = sub as usize;
        if q.contains(&sub) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
