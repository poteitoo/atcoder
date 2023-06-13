use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    };
    let pair = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for i in 0..w {
        for j in 0..h {
            if s[j][i] == '.' {
                let mut cou = 0;
                for (x, y) in &pair {
                    let x = i as isize + x;
                    let y = j as isize + y;
                    if x >= 0 && w as isize > x && y >= 0 && h as isize > y {
                        if s[y as usize][x as usize] == '#' {
                            cou = cou + 1;
                        }
                    }
                }
                if cou > 1 {
                    println!("{} {}", j + 1, i + 1);
                    return;
                }
            }
        }
    }
}
