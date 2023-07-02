use proconio::input;

fn main() {
    input! {
        s: [usize;8]
    }

    if !s.iter().all(|x| x % 25 == 0) {
        println!("No");
        return;
    }
    if !s.iter().all(|x| *x >= 100 && *x <= 675) {
        println!("No");
        return;
    }

    for i in 0..7 {
        if s[i] > s[i + 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
