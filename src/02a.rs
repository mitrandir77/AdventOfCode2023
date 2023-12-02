// Advent of Code 2022
// (c) 2002 Mateusz Kwapich

use anyhow::Result;
use itertools::EitherOrBoth::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut sum = 0;
    for line in stdin.lock().lines() {
        let mut max = BTreeMap::new();
        let line = line?.clone();
        let (header, rest) = line.split_once(':').unwrap();
        let (_game, id) = header.trim().split_once("Game ").unwrap();
        let id: i32 = id.parse().unwrap();
        let sets = rest.split(';');
        for set in sets {
            for color in set.trim().split(',') {
                let (num, color_name) = color.trim().split_once(' ').unwrap();
                let num: i32 = num.parse().unwrap();
                let counter = max.entry(color_name.to_string()).or_insert(0);
                *counter = (*counter).max(num);
            }
        }
        let red = max.get("red").unwrap_or(&0);
        if *red > 12 {
            continue;
        }
        let green = max.get("green").unwrap_or(&0);
        if *green > 13 {
            continue;
        }
        let blue = max.get("blue").unwrap_or(&0);
        if *blue > 14 {
            continue;
        }
        sum += id;
    }

    println!("{sum}");
    Ok(())
}
