use std::time::Instant;

fn main() {
    let data: Vec<i128> = (1..=1_000_000).collect();

    let start_iter = Instant::now();
    let sum_iter: i128 = data.iter().map(|x| x * 2).sum();
    let duration_iter = start_iter.elapsed();
    println!("Iterator Sum: {} in {:?}", sum_iter, duration_iter);

    let start_loop = Instant::now();
    let mut sum_loop = 0;
    for &x in &data {
        sum_loop += x * 2;
    }
    let duration_loop = start_loop.elapsed();
    println!("Loop Sum: {} in {:?}", sum_loop, duration_loop);
}
