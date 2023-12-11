// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use itertools::Itertools;

use std::{collections::BTreeSet, io::BufRead};

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut galaxies = vec![];
    let mut xs = BTreeSet::new();
    let mut x_gaps = BTreeSet::new();
    let mut ys = BTreeSet::new();
    let mut y_gaps = BTreeSet::new();
    for (y, line) in stdin.lock().lines().enumerate() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
                xs.insert(x);
                ys.insert(y);
            }
        }
    }

    for i in *xs.first().unwrap()..*xs.last().unwrap() {
        if !xs.contains(&i) {
            x_gaps.insert(i);
        }
    }
    for i in *ys.first().unwrap()..*ys.last().unwrap() {
        if !ys.contains(&i) {
            y_gaps.insert(i);
        }
    }

    let galaxies = galaxies
        .into_iter()
        .map(|(x, y)| {
            (
                x + x_gaps.range(0..x).count(),
                y + y_gaps.range(0..y).count(),
            )
        })
        .collect_vec();

    let mut sum = 0;
    for ((xa, ya), (xb, yb)) in galaxies.iter().cartesian_product(galaxies.iter()) {
        sum += xa.abs_diff(*xb) + ya.abs_diff(*yb);
    }
    sum /= 2;
    println!("{sum}");
    Ok(())
}
