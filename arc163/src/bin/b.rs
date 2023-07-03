use proconio::input;
// 1. a0 <= *aa  これを満たす最小のインデックスを見つけてleftとする
// 2. *aa <= a1  これを満たす最小のインデックスを見つけてrightとする

fn main() {
    input! {
        n: usize,
        m: usize,
        a0: usize,
        a1: usize,
        a: [usize; n-2]
    }
    let mut num_cal = 0;
    let mut m = m as isize;
    let mut a = a.clone();
    a.sort();

    let mut left_index = a.first().unwrap().clone();
    let mut right_index = a.last().unwrap().clone();
    for (i, aa) in a.iter().enumerate() {
        if a0 <= *aa {
            left_index = i;
            break;
        }
    }
    for (i, aa) in a.iter().enumerate().rev() {
        if a1 >= *aa {
            right_index = i;
            break;
        }
    }
    m -= right_index as isize - left_index as isize + 1;

    for _ in 0..a.len() {
        if m <= 0 {
            println!("{}", num_cal);
            return;
        }

        let next_left_index = left_index as isize - 1;
        let next_right_index = right_index + 1;
        if let Some(next_left_value) = a.get(next_left_index as usize) {
            if let Some(next_right_value) = a.get(next_right_index) {
                let diff_left = a[left_index] - next_left_value;
                let diff_right = next_right_value - a[right_index];
                if diff_left > diff_right {
                    left_index = next_left_index as usize;
                    num_cal += diff_left;
                } else {
                    right_index = next_right_index;
                    num_cal += diff_right;
                }
                m -= 1;
            } else {
                num_cal += a[left_index] - next_left_value;
                left_index = next_left_index as usize;
                m -= 1;
            }
        } else {
            if let Some(next_right_value) = a.get(next_right_index) {
                num_cal += next_right_value - a[right_index];
                right_index = next_right_index;
                m -= 1;
            }
        }
    }

    println!("{}", num_cal);
}
