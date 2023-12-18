// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use std::io::BufRead;

#[macro_use]
extern crate scan_rules;
use scan_rules::{
    scan,
    scanner::{Hex, Number, Word},
};

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut y = 1000000;
    let mut sum = 1;
    for line in stdin.lock().lines() {
        let line = line?;
        // dbg!(&line);
        let (dir, steps) = scan!(line.as_str();
            (let dir: Word, " " , let steps: Number, " (#", let _colour: Hex<i64>, ")") => {
                (dir, steps.parse::<i64>().unwrap())
                // dbg!(dir,steps);
            }
        )
        .unwrap();

        match dir {
            "U" => {
                y += steps;
            }
            "D" => {
                y -= steps;
                sum += steps;
            }
            "L" => {
                sum -= (y) * (steps);
                //x += steps;
            }
            "R" => {
                sum += (y + 1) * (steps);
                //x -= steps;
            }
            _ => panic!("wrong direction {}", dir),
        }
    }

    println!("{sum}");
    Ok(())
}
