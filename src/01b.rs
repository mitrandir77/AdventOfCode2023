// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut sum = 0;

    for line in stdin.lock().lines() {
        let line = line?
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");

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
