use proconio::input;

const SUM_BALL: usize = 465;
const NUM_BALL_ROW: usize = 30;

fn main() {
    input! {
        data: [usize;SUM_BALL]
    }

    let mut balls: [[usize; NUM_BALL_ROW]; NUM_BALL_ROW] = [[0; NUM_BALL_ROW]; NUM_BALL_ROW];
    for i in 0..30 {
        for j in 0..i + 1 {
            let index = map_to_original_index(i, j);
            balls[i][j] = data[index];
        }
    }
}

fn map_to_original_index(i: usize, j: usize) -> usize {
    (i * i + i + 1) / 2 + j
}

fn swap_if_parent_larger(
    balls: &mut Vec<Vec<usize>>,
    parent_pos: (usize, usize),
    child_pos_1: (usize, usize),
    child_pos_2: (usize, usize),
) {
    let parent = balls[parent_pos.0][parent_pos.1];
    let child_1 = balls[child_pos_1.0][child_pos_1.1];
    let child_2 = balls[child_pos_2.0][child_pos_2.1];

    if parent > child_1 {
        if parent > child_2 {}
    } else {
        if parent > child_2 {}
    }
}

fn get_parents_positions(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let x1 = pos.0 as isize - 1;
    let y1 = pos.1 as isize - 1;

    let x2 = pos.0 as isize - 1;
    let y2 = pos.1 as isize;

    if x1 >= 0 && y1 >= 0 {
        res.push((x1 as usize, y1 as usize));
    }
    if x2 >= 0 && y2 >= 0 {
        res.push((x2 as usize, y2 as usize));
    }
    res
}
