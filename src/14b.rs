// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::io::BufRead;

fn tilt_west(map: &Vec<String>) -> Vec<String> {
    map.iter()
        .map(|row| {
            row.split('#')
                .map(|sect| {
                    let mut v = sect.chars().collect_vec();
                    v.sort();
                    v.reverse();
                    v.iter().collect::<String>()
                })
                .join("#")
        })
        .collect()
}

fn transpose(map: &Vec<String>) -> Vec<String> {
    let mut transposed = vec![String::new(); map[0].len()];
    for row in map.iter() {
        for (i, c) in row.chars().enumerate() {
            transposed[i].push(c);
        }
    }
    transposed
}

fn reverse(map: &Vec<String>) -> Vec<String> {
    map.iter().map(|s| s.chars().rev().collect()).collect_vec()
}

fn tilt_north(map: &Vec<String>) -> Vec<String> {
    transpose(&tilt_west(&transpose(map)))
}

fn tilt_south(map: &Vec<String>) -> Vec<String> {
    transpose(&reverse(&tilt_west(&reverse(&transpose(map)))))
}

fn tilt_east(map: &Vec<String>) -> Vec<String> {
    reverse(&tilt_west(&reverse(map)))
}

fn load(map: &Vec<String>) -> usize {
    let len = map[0].len();
    let map = transpose(&map);
    map.iter()
        .map(|row| {
            row.chars()
                .enumerate()
                .filter_map(|(i, c)| if c == 'O' { Some(len - i) } else { None })
                .sum::<usize>()
        })
        .sum()
}

fn cycle(map: &Vec<String>) -> Vec<String> {
    let map = tilt_north(map);
    let map = tilt_west(&map);
    let map = tilt_south(&map);
    tilt_east(&map)
}
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() -> Result<()> {
    let mut map = vec![];
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        map.push(line);
    }

    let mut seen = BTreeMap::new();
    for i in 0.. {
        let new_map = cycle(&map);
        let hash = calculate_hash(&new_map);
        map = new_map;
        if let Some(prev_i) = seen.get(&hash) {
            let cycle_len = i - prev_i;
            let pos = (1000000000 - i) % cycle_len;

            for _i in 0..pos-1 {
                map = cycle(&map);
            }
            let sum = load(&map);
            println!("{sum}");
            return Ok(());
        }
        seen.insert(hash, i);
    }
    Ok(())
}
