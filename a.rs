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

    for i in (1..30).rev() {
        for j in 0..i {
            let child_1 = balls[i][j];
            let child_2 = balls[i][j + 1];

            let parent_1 = balls[i - 1][j];
        }
    }
}

fn is_adjacent(pos1: (usize, usize), pos2: (usize, usize)) -> bool {
    if pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1 - 1 {
        true
    } else if pos1.0 == pos2.0 - 1 && pos1.1 == pos2.1 {
        true
    } else if pos1.0 == pos2.0 && pos1.1 == pos2.1 - 1 {
        true
    } else if pos1.0 == pos2.0 && pos1.1 == pos2.1 + 1 {
        true
    } else if pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1 {
        true
    } else if pos1.0 == pos2.0 + 1 && pos1.1 == pos2.1 + 1 {
        true
    } else {
        false
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

fn get_parents() {}
