use proconio::input_interactive;

fn main() {
    let mut arr = vec![];
    for _ in 0..100 {
        input_interactive!(a: usize);
        arr.push(a);
        if a == 0 {
            break;
        }
    }
    arr.reverse();

    for a in arr {
        println!("{}", a);
    }
}
