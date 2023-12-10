// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use itertools::Itertools;

use std::{collections::BTreeSet, io::BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}
use PipeType::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Pipe(PipeType),
    Ground,
    Start,
}
use Tile::*;

fn follow_pipe(
    (px, py): (usize, usize),
    (x, y): (usize, usize),
    tile: Tile,
) -> Option<(usize, usize)> {
    let pipe = if let Pipe(pipe) = tile {
        pipe
    } else {
        return None;
    };
    // dbg!(&pipe, (x,y), (px, py));
    match pipe {
        NS => {
            if py + 1 == y && px == x {
                Some((x, y + 1))
            } else if py.checked_sub(1) == Some(y) && px == x {
                Some((x, y - 1))
            } else {
                None
            }
        }
        EW => {
            if py == y && px + 1 == x {
                Some((x + 1, y))
            } else if py == y && px.checked_sub(1) == Some(x) {
                Some((x - 1, y))
            } else {
                None
            }
        }
        NE => {
            if py == y && px.checked_sub(1) == Some(x) {
                Some((x, y - 1))
            } else if py + 1 == y && px == x {
                Some((x + 1, y))
            } else {
                None
            }
        }
        NW => {
            if py == y && px + 1 == x {
                Some((x, y - 1))
            } else if py + 1 == y && px == x {
                Some((x - 1, y))
            } else {
                None
            }
        }
        SW => {
            if py == y && px + 1 == x {
                Some((x, y + 1))
            } else if py.checked_sub(1) == Some(y) && px == x {
                Some((x - 1, y))
            } else {
                None
            }
        }
        SE => {
            if py == y && px.checked_sub(1) == Some(x) {
                Some((x, y + 1))
            } else if py.checked_sub(1) == Some(y) && px == x {
                Some((x + 1, y))
            } else {
                None
            }
        }
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut map = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        map.push(
            line.chars()
                .map(|c| match c {
                    '|' => Pipe(NS),
                    '-' => Pipe(EW),
                    'L' => Pipe(NE),
                    'J' => Pipe(NW),
                    '7' => Pipe(SW),
                    'F' => Pipe(SE),
                    '.' => Ground,
                    'S' => Start,
                    _ => panic!("unexpected tile!"),
                })
                .collect_vec(),
        );
    }

    let (mut sx, mut sy) = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Start {
                (sx, sy) = (x, y);
            }
        }
    }

    let (mut px, mut py) = (sx, sy);
    let (mut x, mut y) = (0, 0);

    let (isx, isy) = (sx as i64, sy as i64);
    let cand: Vec<(usize, usize)> = [
        (isx + 1, isy),
        (isx - 1, isy),
        (isx, isy + 1),
        (isx, isy - 1),
        (isx - 1, isy + 1),
        (isx - 1, isy + 1),
        (isx + 1, isy + 1),
        (isx + 1, isy - 1),
    ]
    .into_iter()
    .filter(|(x, y)| *x >= 0 && *y >= 0)
    .map(|(x, y)| (x as usize, y as usize))
    .collect_vec();
    for (cx, cy) in cand {
        if follow_pipe((px, py), (cx, cy), map[cy][cx]).is_some() {
            (x, y) = (cx, cy);
        }
    }

    let mut visited = BTreeSet::new();
    visited.insert((sx, sy));
    while (x, y) != (sx, sy) {
        visited.insert((x, y));
        let (nx, ny) = follow_pipe((px, py), (x, y), map[y][x]).unwrap();
        (px, py) = (x, y);
        (x, y) = (nx, ny);
    }
    let mut cnt = 0;

    for (y, row) in map.iter().enumerate() {
        let mut prev = 0;
        let mut prev_visited = None;
        for (x, tile) in row.iter().enumerate() {
            if visited.contains(&(x, y)) {
                let c = match *tile {
                    Start => 'S',
                    Pipe(NS) => '│',
                    Pipe(EW) => '─',
                    Pipe(SE) => '┌',
                    Pipe(NE) => '└',
                    Pipe(NW) => '┘',
                    Pipe(SW) => '┐',
                    _ => '#',
                };
                print!("{c}");
                if *tile == Pipe(NS) {
                    prev += 1;
                    prev_visited = None;
                } else if *tile != Pipe(EW) {
                    if let Some(pv) = prev_visited {
                        if (*tile == Pipe(SE) && pv == Pipe(NW))
                            || (*tile == Pipe(NE) && pv == Pipe(SW))
                            || (*tile == Pipe(SW) && pv == Pipe(NE))
                            || (*tile == Pipe(NW) && pv == Pipe(SE))
                        {
                            prev += 1;
                        }
                        prev_visited = None;
                    } else {
                        prev_visited = Some(*tile);
                    }
                }
                continue;
            }
            prev_visited = None;

            if prev % 2 == 1 {
                cnt += 1;
                print!("I");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("{cnt}");
    Ok(())
}
