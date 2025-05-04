use std::thread;

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        result.extend_from_slice(&left[i..]);
    }
    if j < right.len() {
        result.extend_from_slice(&right[j..]);
    }

    result
}

fn merge_sort(seq: &[i32]) -> Vec<i32> {
    if seq.len() <= 1 {
        return seq.to_vec();
    }

    let mid = seq.len() / 2;
    let left = merge_sort(&seq[..mid]);
    let right = merge_sort(&seq[mid..]);

    merge(&left, &right)
}

fn parallel_merge_sort(seq: &[i32]) -> Vec<i32> {
    println!("sorting: {:?}", seq);
    const THRESHOLD: usize = 2;

    if seq.len() <= 1 {
        return seq.to_vec();
    }

    if seq.len() <= THRESHOLD {
        return merge_sort(seq); // Use sequential for small slices
    }

    let mid = seq.len() / 2;
    let left = seq[..mid].to_vec();
    let right = seq[mid..].to_vec();

    let left_handle = thread::spawn(move || parallel_merge_sort(&left));
    let right_handle = thread::spawn(move || parallel_merge_sort(&right));

    let left_sorted = left_handle.join().expect("Left thread panicked");
    let right_sorted = right_handle.join().expect("Right thread panicked");

    merge(&left_sorted, &right_sorted)
}

fn main() {
    let data = vec![38, 27, 43, 3, 9, 82, 10, -3];
    println!("Original: {:?}", data);

    let sorted_seq = merge_sort(&data);
    println!("Sequentially sorted: {:?}", sorted_seq);

    let sorted_par = parallel_merge_sort(&data);
    println!("Parallel sorted: {:?}", sorted_par);
}
