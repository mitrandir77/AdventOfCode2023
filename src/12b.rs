// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use itertools::Itertools;
use memoize::memoize;

use std::io::BufRead;

#[memoize]
fn count_start(springs: String, groups: Vec<usize>) -> usize {
    if springs.is_empty() {
        if groups.is_empty() {
            return 1;
        }
        return 0;
    }
    if groups.iter().sum::<usize>() + groups.iter().len() > springs.len() + 1 {
        return 0;
    }

    match &springs[0..1] {
        "." => count_start(springs[1..].to_string(), groups),
        "#" => {
            let mut groups = groups.to_vec();
            if groups.is_empty() {
                return 0;
            }
            assert!(groups[0] > 0);
            groups[0] -= 1;
            count_cont(springs[1..].to_string(), groups)
        }
        "?" => {
            let start = count_start(springs[1..].to_string(), groups.clone());
            if groups.is_empty() {
                return start;
            }
            let mut new_groups = groups.to_vec();
            assert!(new_groups[0] > 0);
            new_groups[0] -= 1;
            count_cont(springs[1..].to_string(), new_groups) + start
        }
        _ => panic!("unexpected char"),
    }
}

#[memoize]
fn count_cont(springs: String, groups: Vec<usize>) -> usize {
    if springs.is_empty() {
        if groups.is_empty() {
            return 1;
        }
        if groups[0] == 0 {
            return 1;
        }
        return 0;
    }
    if groups.iter().sum::<usize>() + groups.iter().len() - 1 > springs.len() {
        return 0;
    }

    match &springs[0..1] {
        "." => {
            if groups.is_empty() {
                1
            } else if groups[0] == 0 {
                count_start(springs.to_string(), groups[1..].to_vec())
            } else {
                0
            }
        }
        "#" => {
            if groups.is_empty() {
                0
            } else if groups[0] > 0 {
                let mut groups = groups.to_vec();
                groups[0] -= 1;
                count_cont(springs[1..].to_string(), groups)
            } else {
                0
            }
        }
        "?" => {
            if groups.is_empty() {
                count_start(springs[1..].to_string(), groups)
            } else if groups[0] > 0 {
                let mut groups = groups.to_vec();
                groups[0] -= 1;
                count_cont(springs[1..].to_string(), groups)
            } else {
                count_start(springs[1..].to_string(), groups[1..].to_vec())
            }
        }
        _ => panic!("unexpected char"),
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut sum: usize = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let (springs, groups) = line.split_once(' ').unwrap();

        let groups = groups
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();
        let mut springs = springs.to_string();
        springs.push('?');
        let mut springs = springs.repeat(5);
        springs.pop().unwrap();
        sum += count_start(springs, groups.repeat(5));
    }

    println!("{sum}");
    Ok(())
}
