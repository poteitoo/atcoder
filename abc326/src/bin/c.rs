use proconio::input;

fn find_max_index_less_than(arr: &[usize], b: usize) -> Option<usize> {
    if arr.is_empty() || arr[0] >= b {
        return None;
    }

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] < b {
            if mid == arr.len() - 1 || arr[mid + 1] >= b {
                return Some(mid);
            }
            left = mid + 1;
        } else {
            if mid == 0 {
                return None;
            }
            right = mid - 1;
        }
    }

    None
}

fn find_max_elements(arr: &mut Vec<usize>, m: usize) -> usize {
    let mut max_count = 0;

    arr.sort_unstable();
    for (i, &x) in arr.iter().enumerate() {
        let end = x + m;
        let count = find_max_index_less_than(arr, end).unwrap() - i + 1;

        if count > max_count {
            max_count = count;
        }
    }

    max_count
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n]
    }

    let max_count = find_max_elements(&mut a, m);
    println!("{}", max_count);
}
