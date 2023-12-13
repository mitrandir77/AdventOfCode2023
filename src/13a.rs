// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use std::io::BufRead;

fn compute_horizontal_reflection(map: Vec<String>) -> u64 {
    let mut reverse =map.clone();
    reverse.reverse();
    let map_len = map.len();
    for i in 1..map_len {
        let reflection_len = i.min(map_len-i);
        // let a = &map[i-reflection_len..i];
        // let b = &reverse[map_len-(i+reflection_len)..map_len-i];
        // dbg!(a,b);
        if map[i-reflection_len..i] == reverse[map_len-(i+reflection_len)..map_len-i] {
            return i as u64;
        }
    }
    0
}

fn compute_reflections(map: Vec<String>) -> u64 {
    let mut transposed = vec![String::new(); map[0].len()];
    for row in map.iter() {
        for (i,c) in row.chars().enumerate() {
            transposed[i].push(c);
        }
    }
    compute_horizontal_reflection(map) * 100 + compute_horizontal_reflection(transposed)
}

fn main() -> Result<()> {
    let mut map = vec![];
    let mut sum: u64 = 0;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            sum += compute_reflections(map);
            map = vec![];
            continue;
        }
        map.push(line);
    }
    sum += compute_reflections(map);

    println!("{sum}");
    Ok(())
}
