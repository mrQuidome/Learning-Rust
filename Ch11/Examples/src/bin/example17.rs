fn main() {
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];

    let grand_total = nested
        .into_iter()
        .flatten()
        .sum::<i32>();
    println!("Grand total: {}", grand_total);
}