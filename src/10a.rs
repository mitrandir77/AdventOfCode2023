// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use itertools::Itertools;

use std::io::BufRead;

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
                Some((x, y+ 1))
            } else if py.checked_sub(1) == Some(y) && px == x {
                Some((x, y-1))
            } else {
                None
            }
        }
        EW => {
            if py == y && px + 1 == x {
                Some((x+1, y))
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
            } else if py+1 ==y && px == x {
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

    // dbg!(sx, sy);
    let mut cnt = 0;
    while (x, y) != (sx, sy) {
        // dbg!(cnt);
        let (nx, ny) = follow_pipe((px, py), (x, y), map[y][x]).unwrap();
        (px, py) = (x, y);
        (x, y) = (nx, ny);
        cnt += 1;
    }

    let res = (cnt + 1)/2;

    println!("{res}");
    Ok(())
}
