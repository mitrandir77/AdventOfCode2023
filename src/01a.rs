// Advent of Code 2023
// (c) 2023 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let mut digits = vec![];
        for c in line.chars() {
            if let Some(num) = c.to_digit(10) {
                digits.push(num);
            }
        }

        let a = digits.first().unwrap();
        let b = digits.last().unwrap();
        let c = a * 10 + b;
        sum += c;
    }
    println!("{sum}");
    Ok(())
}
