use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: [[usize; 9]; 9]
    }

    let mut yoko = HashSet::new();
    let mut tate = HashSet::new();
    for i in 0..9 {
        for j in 0..9 {
            yoko.insert(a[i][j]);
            tate.insert(a[j][i]);
        }
        if yoko.len() == 9 && tate.len() == 9 {
            yoko.clear();
            tate.clear();
        } else {
            println!("No");
            return;
        }
    }

    let mut block = HashSet::new();
    for i in 0..3 {
        for j in 0..3 {
            for x in 0..3 {
                for y in 0..3 {
                    block.insert(a[3 * i + x][3 * j + y]);
                }
            }
            if block.len() == 9 {
                block.clear();
            } else {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
