use proconio::input;

fn main() {
    input! {
        abc: usize
    }

    let mut ans = Vec::new();
    for i in 1..10 {
        for j in 0..10 {
            for k in 0..10 {
                if i * j == k {
                    let num = i * 100 + j * 10 + i * j;
                    ans.push(num);
                }
            }
        }
    }
    ans.sort_unstable();
    let pos = ans.iter().find(|p| **p >= abc).unwrap();
    println!("{}", pos);
}
