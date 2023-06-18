use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        a: [usize; 3*n]
    }

    // i：j、すでに値が入っている
    let mut map: HashMap<&usize, (usize, bool)> = HashMap::new();
    for (i, aa) in a.iter().enumerate() {
        if let Some(v) = map.get_mut(aa) {
            if !v.1 {
                v.0 = i + 1;
                v.1 = true;
            }
        } else {
            map.insert(aa, (0, false));
        }
    }
    let mut res: Vec<(usize, usize)> = (1..n + 1)
        .map(|i| (i, map.get(&i).unwrap().0))
        .collect_vec();

    res.sort_by_key(|&(_, a)| a);

    let res: String = res
        .iter()
        .map(|&(x, _)| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", res);
}
