use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        ha:usize, _:usize,
        a:[String; ha],
        hb:usize, _:usize,
        b:[String; hb],
        hx:usize, wx:usize,
        x:[String; hx],
    }

    let black_a = find_black_indices(&a);
    let black_b = find_black_indices(&b);
    let black_x = find_black_indices(&x);
    let black_a = calc_centered_pos(&black_a);
    let black_b = calc_centered_pos(&black_b);
    let black_x = calc_centered_pos(&black_x);

    let mut is_subset = false;
    let mut offset_a = (0, 0);
    'outer1: for iy in 0..hx {
        for ix in 0..wx {
            if black_a
                .iter()
                .all(|v| black_x.contains(&(ix + v.0, iy + v.1)))
            {
                is_subset = true;
                offset_a = (ix, iy);
                break 'outer1;
            }
        }
    }
    if !is_subset {
        return println!("No");
    }

    let mut is_subset = false;
    let mut offset_b = (0, 0);
    'outer2: for iy in 0..hx {
        for ix in 0..wx {
            if black_b
                .iter()
                .all(|v| black_x.contains(&(ix + v.0, iy + v.1)))
            {
                is_subset = true;
                offset_b = (ix, iy);
                break 'outer2;
            }
        }
    }
    if !is_subset {
        return println!("No");
    }

    let set_a: HashSet<(usize, usize)> = black_a
        .iter()
        .map(|p| (p.0 + offset_a.0, p.1 + offset_a.1))
        .collect();
    let set_b: HashSet<(usize, usize)> = black_b
        .iter()
        .map(|p| (p.0 + offset_b.0, p.1 + offset_b.1))
        .collect();
    let set_x: HashSet<_> = HashSet::from(&black_x);

    let diff_set = (set_a.union(set_b)).difference(set_x);
    if diff_set.len() == 0 {
        return println!("Yes");
    }

    println!("No");
}

fn find_black_indices(sheet: &Vec<String>) -> Vec<(usize, usize)> {
    let mut indices: Vec<(usize, usize)> = Vec::new();
    for (row, line) in sheet.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                indices.push((row, col));
            }
        }
    }

    indices
}

fn find_least_index(arr: &Vec<(usize, usize)>) -> (usize, usize) {
    let min_row = arr[0].0;
    let mut min_col = arr[0].1;

    for (_, col) in arr.iter() {
        if *col < min_col {
            min_col = *col;
        }
    }

    (min_row, min_col)
}

fn calc_centered_pos(arr: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let least = find_least_index(arr);
    arr.into_iter()
        .map(|&(a, b)| (a - least.0, b - least.1))
        .collect()
}
