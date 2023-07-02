use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p0: usize,
        p: [usize; m]
    }
    let mut sum = 0;
    let mut price_map: HashMap<String, usize> = HashMap::new();
    for i in 0..m {
        price_map.insert(d[i].clone(), p[i]);
    }
    for color in c.iter().cloned() {
        if let Some(price) = price_map.get(&color) {
            sum += *price;
        } else {
            sum += p0;
        }
    }
    println!("{}", sum);
}
