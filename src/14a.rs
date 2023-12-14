// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use itertools::Itertools;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut map = vec![];
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        map.push(line);
    }

    let mut transposed = vec![String::new(); map[0].len()];
    for row in map.iter() {
        for (i, c) in row.chars().enumerate() {
            transposed[i].push(c);
        }
    }

    let len = transposed[0].len();
    let sum: usize = transposed
        .iter()
        .map(|col| {
            col.split('#')
                .map(|sect| {
                    let mut v = sect.chars().collect_vec();
                    v.sort();
                    v.reverse();
                    v.iter().collect::<String>()
                })
                .join("#")
                .chars()
                .enumerate()
                .filter_map(|(i, c)| if c == 'O' { Some(len - i) } else { None })
                .sum::<usize>()
        })
        .sum();
    println!("{sum}");
    Ok(())
}
