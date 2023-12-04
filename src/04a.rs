// Advent of Code 2023
// (c) 2023 Mateusz Kwapich

use anyhow::Result;
use std::collections::BTreeSet;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut sum: u64 = 0;
    for line in stdin.lock().lines() {
        let line = line?.clone();
        let (_header, rest) = line.split_once(':').unwrap();
        let (winning, my) = rest.split_once('|').unwrap();
        let winning: BTreeSet<u64> = winning
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let count = my
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .filter(|n| winning.contains(n))
            .count();
        if count >= 1 {
            sum += 1 << (count - 1);
        }
    }

    println!("{sum}");
    Ok(())
}
