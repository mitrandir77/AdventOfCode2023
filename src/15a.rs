// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use itertools::Itertools;
use std::io::Read;

fn hash(input: &str) -> u32 {
    let mut val = 0;
    for c in input.chars() {
        val += c as u32;
        val *= 17;
        val %= 256;
    }
    val
}

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input.trim().split(',').collect_vec();

    let mut sum = 0;
    for op in input {
        sum += hash(op);
    }

    println!("{sum}");
    Ok(())
}
