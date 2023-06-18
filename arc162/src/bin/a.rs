use proconio::input;

fn main() {
    input! {
        t:usize
    }

    for _ in 0..t {
        input! {n:usize, p:[isize;n]}
        let drop_num = p
            .iter()
            .enumerate()
            .map(|(i, x)| *x - i as isize - 1)
            .filter(|x| *x < 0)
            .count();

        if drop_num == 0 {
            println!("{}", n);
        } else {
            println!("{}", drop_num);
        }
    }
}
