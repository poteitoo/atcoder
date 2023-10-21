use proconio::{input, marker::Chars};

fn dfs(x: usize, y: usize, n: usize, m: usize, field: &mut Vec<Vec<char>>) {
    let neighbors = [-1, 0, 1];
    field[x][y] = '.';
    for &dx in &neighbors {
        for &dy in &neighbors {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0
                && nx < n as isize
                && ny >= 0
                && ny < m as isize
                && field[nx as usize][ny as usize] == '#'
            {
                dfs(nx as usize, ny as usize, n, m, field);
            }
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut field: [Chars; h]
    }

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if field[i][j] == '#' {
                dfs(i, j, h, w, &mut field);
                res += 1;
            }
        }
    }

    println!("{}", res);
}
