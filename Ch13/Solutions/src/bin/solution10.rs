use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    const PLAYER_COUNT: usize = 6;

    // Shared leaderboard
    let leaderboard: Arc<RwLock<HashMap<String, u32>>> =
        Arc::new(RwLock::new(HashMap::new()));

    let mut handles = vec![];

    for i in 0..PLAYER_COUNT {
        let leaderboard_clone = Arc::clone(&leaderboard);
        let player_name = format!("Player{}", i + 1);

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 * i as u64)); // Simulate staggered activity

            if i % 2 == 0 {
                // Writer thread: update the leaderboard
                let score = (i as u32 + 1) * 10;
                {
                    let mut board = leaderboard_clone.write().unwrap();
                    board.insert(player_name.clone(), score);
                    println!("{} submitted a score of {}", player_name, score);
                }
            } else {
                // Reader thread: read the leaderboard
                {
                    let board = leaderboard_clone.read().unwrap();
                    let mut entries: Vec<_> = board.iter().collect();
                    entries.sort_by(|a, b| b.1.cmp(a.1)); // Sort by score descending

                    println!("{} is reading the leaderboard:", player_name);
                    for (name, score) in entries.iter().take(3) {
                        println!("  {}: {}", name, score);
                    }
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Final leaderboard state
    println!("\nFinal Leaderboard:");
    let board = leaderboard.read().unwrap();
    for (name, score) in board.iter() {
        println!("{}: {}", name, score);
    }
}
