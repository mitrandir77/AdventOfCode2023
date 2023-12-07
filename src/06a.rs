// Advent of Code 2023
// (c) 2023 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap()?;
    let (header, rest) = line.split_once(':').unwrap();
    assert_eq!(header, "Time");
    let times: Vec<u64> = rest
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    let line = lines.next().unwrap()?;
    let (header, rest) = line.split_once(':').unwrap();
    assert_eq!(header, "Distance");
    let distances: Vec<u64> = rest
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut res: u64 = 1;
    for (time, record_distance) in times.into_iter().zip(distances.into_iter()) {
        let mut cnt: u64 = 0;
        for speed in 1..time {
            let travel_time = time - speed;
            let travel_distance = travel_time * speed;
            if travel_distance > record_distance {
                cnt += 1;
            }
        }
        res *= cnt;
    }
    println!("{res}");

    Ok(())
}
