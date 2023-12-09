// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;

use std::io::BufRead;

fn extrapolation(mut seq: Vec<i64>) -> Vec<i64> {
    if seq.iter().all(|val| *val == 0)  {
        seq.push(0);
        seq
    } else {
        let diff = seq.iter().skip(1).zip(seq.iter()).map(|(a, b)| a - b).collect();
        let new_diff = extrapolation(diff);
        seq.push(seq.last().unwrap() + new_diff.last().unwrap());
        seq
    }
}


fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let seq: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let extrapolated = extrapolation(seq);
        sum += extrapolated.last().unwrap();
    }

    println!("{sum}");
    Ok(())
}
