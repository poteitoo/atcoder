use proconio::{input, marker::Chars};

fn find_possible_starts(grid: &Vec<Vec<char>>, path: String) -> Vec<(usize, usize)> {
    // Convert the path to reverse and opposite directions
    let reverse_path: String = path
        .chars()
        .rev()
        .map(|c| match c {
            'L' => 'R',
            'R' => 'L',
            'U' => 'D',
            'D' => 'U',
            _ => c,
        })
        .collect();

    let mut possible_starts = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    // Check if a position is valid
    let is_valid_position = |r: isize, c: isize| {
        r >= 0
            && c >= 0
            && (r as usize) < rows
            && (c as usize) < cols
            && grid[r as usize][c as usize] == '.'
    };

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '.' {
                let mut r = i as isize;
                let mut c = j as isize;

                for step in reverse_path.chars() {
                    match step {
                        'L' => c -= 1,
                        'R' => c += 1,
                        'U' => r -= 1,
                        'D' => r += 1,
                        _ => {}
                    }

                    if !is_valid_position(r, c) {
                        break;
                    }
                }

                if is_valid_position(r, c) {
                    possible_starts.push((i, j));
                }
            }
        }
    }

    possible_starts
}

fn main() {
    input! {
        h: usize,
        _: usize,
        __: usize,
        t: String,
        s: [Chars; h]
    }

    let starts = find_possible_starts(&s, t);
    println!("{}", starts.len());
}
