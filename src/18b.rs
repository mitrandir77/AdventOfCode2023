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
            (let _dir: Word, " " , let _steps: Number, " (#", let colour: Hex<i64>, ")") => {
                let dir = match colour %16 {
                    0 => "R",
                    1 => "D",
                    2 => "L",
                    3 => "U",
                    _ => panic!("wrong direction {}", colour %16),
                };
                (dir, colour/16)
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
