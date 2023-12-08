// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
#![feature(btree_cursors)]

use anyhow::Result;
#[macro_use]
extern crate scan_rules;
use scan_rules::scan;
use scan_rules::scanner::Word;

use std::collections::HashMap;
use std::io::BufRead;


fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let instruction_line = stdin.lock().lines().next().unwrap()?;
    let mut graph = HashMap::new();

    for line in stdin.lock().lines().skip(1) {
        let line = line?;
        scan!(&line;
            (let from: Word, " = (", let left: Word, ", ", let right: Word, ")") => {
                graph.insert(from.to_owned(), (left.to_owned(), right.to_owned()));
            },
        )
        .unwrap();
    }

    let mut cur_positions: Vec<_> = graph.keys().filter(|k| k.ends_with('A')).collect();
    let mut steps = 0;
    for instruction in instruction_line.chars().cycle() {
        if cur_positions.iter().all(|k| k.ends_with('Z')) {
            break;
        }
        cur_positions = cur_positions.into_iter().map(|pos| {
            let edges = graph.get(pos).unwrap();
            match instruction {
                'L' => {
                    &edges.0
                }
                'R' => {
                    &edges.1
                }
                _ => {
                    panic!("wrong instruction!");
                }
            }
        }).collect();

        steps += 1;
    }

    println!("{steps}");
    Ok(())
}
