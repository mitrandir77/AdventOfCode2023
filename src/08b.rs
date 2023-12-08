// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
#[macro_use]
extern crate scan_rules;
use scan_rules::scan;
use scan_rules::scanner::Word;

use std::collections::HashMap;
use std::io::BufRead;
use  num::integer::lcm;

fn steps(start: &str, graph: &HashMap<String, (String, String)>, instructions: &str,) -> u64 {
    let mut pos = start;
    let mut steps: u64 = 0;
    for instruction in instructions.chars().cycle() {
        if pos.ends_with('Z'){
            break;
        }
        let edges = graph.get(pos).unwrap();
        pos = match instruction {
            'L' => {
                &edges.0
            }
            'R' => {
                &edges.1
            }
            _ => {
                panic!("wrong instruction!");
            }
        };
        steps += 1;
    }
    steps

}

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

    let cycles: Vec<_> = graph.keys().filter(|k| k.ends_with('A')).map(
        |pos| steps(pos, &graph, &instruction_line)
    ).collect();

    let lcm = cycles.into_iter().fold(1, lcm);


    println!("{lcm}");
    Ok(())
}
