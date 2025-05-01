fn main() {
    let data: Vec<i128> = (1..=1_000_000).collect();

    let start_collect = std::time::Instant::now();
    let sum_evens = data
        .iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect::<Vec<i128>>()
        .iter()
        .sum::<i128>();
    let duration_collect = start_collect.elapsed();
    println!(
        "Using `collect`: Sum of evens is {} in {:?}",
        sum_evens, duration_collect
    );

    let start_direct = std::time::Instant::now();
    let sum_evens_direct = data.iter().filter(|&&x| x % 2 == 0).sum::<i128>();
    let duration_direct = start_direct.elapsed();
    println!(
        "Direct sum of evens: {} in {:?}",
        sum_evens_direct, duration_direct
    );
}
