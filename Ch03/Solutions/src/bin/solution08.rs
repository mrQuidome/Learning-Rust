fn main() {
    let mut numbers = [1, 2, 3, 4, 5, 6, 7, 8];

    // split_at_mut safely splits a mutable slice into two disjoint mutable slices.
    // This is necessary because Rust normally prevents having two mutable references
    // to the same array at the same time.
    // split_at_mut guarantees at compile time that the two slices do not overlap.
    let (slice1, slice2) = numbers.split_at_mut(4);

    // Now we can safely mutate each part independently, knowing there's no overlap.
    slice1[0] = 100;
    slice2[0] = 101;

    // We print the entire array to verify the changes made through both slices.
    println!("{numbers:?}");
}