fn insertion_sort(arr: &mut [i32]) -> usize {
    let mut count = 0;

    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
            count += 1;
        }

        arr[j] = key;
    }

    count
}

fn main() {
    let mut numbers = vec![5, 2, 4, 6, 1, 3];
    let tries = insertion_sort(&mut numbers);

    println!("Number of tries: {}", tries);
    println!("Sorted numbers: {:?}", numbers);
}
