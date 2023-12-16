// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use itertools::Itertools;
use std::collections::{BTreeSet, VecDeque};
use std::io::BufRead;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(usize, usize);
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Point {
    type Output = Option<Point>;
    fn sub(self, other: Point) -> Option<Point> {
        match (self.0.checked_sub(other.0), self.1.checked_sub(other.1)) {
            (Some(x), Some(y)) => Some(Point(x, y)),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Point {
    fn mov(self, dir: Dir) -> Option<Point> {
        match dir {
            Dir::Up => self - Point(0, 1),
            Dir::Down => Some(self + Point(0, 1)),
            Dir::Left => self - Point(1, 0),
            Dir::Right => Some(self + Point(1, 0)),
        }
    }
}

fn maybe_enqueue(
    point: Option<Point>,
    dir: Dir,
    queue: &mut VecDeque<(Point, Dir)>,
    visited: &mut BTreeSet<(Point, Dir)>,
) {
    if let Some(point) = point {
        if !visited.contains(&(point, dir)) {
            visited.insert((point, dir));
            queue.push_back((point, dir));
        }
    }
}
fn main() -> Result<()> {
    let mut map = vec![];
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        map.push(line.chars().collect_vec());
    }
    let mut q = VecDeque::from([(Point(0, 0), Dir::Right)]);
    let mut visited = BTreeSet::new();
    visited.insert((Point(0,0), Dir::Right));

    while let Some((p, d)) = q.pop_front() {
        match map.get(p.1).map(|l| l.get(p.0)) {
            Some(Some('.')) => {
                maybe_enqueue(p.mov(d), d, &mut q, &mut visited);
            }
            Some(Some('\\')) => {
                let d = match d {
                    Dir::Down => Dir::Right,
                    Dir::Up => Dir::Left,
                    Dir::Right => Dir::Down,
                    Dir::Left => Dir::Up,
                };
                maybe_enqueue(p.mov(d), d, &mut q, &mut visited);
            }
            Some(Some('/')) => {
                let d = match d {
                    Dir::Down => Dir::Left,
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Up,
                    Dir::Left => Dir::Down,
                };
                maybe_enqueue(p.mov(d), d, &mut q, &mut visited);
            }
            Some(Some('|')) => {
                match d {
                    Dir::Down | Dir::Up => {
                        maybe_enqueue(p.mov(d), d, &mut q, &mut visited);
                    }
                    Dir::Right | Dir::Left => {
                        maybe_enqueue(p.mov(Dir::Up), Dir::Up, &mut q, &mut visited);
                        maybe_enqueue(p.mov(Dir::Down), Dir::Down, &mut q, &mut visited);
                    }
                };
            }
            Some(Some('-')) => {
                match d {
                    Dir::Right | Dir::Left => {
                        maybe_enqueue(p.mov(d), d, &mut q, &mut visited);
                    }
                    Dir::Down | Dir::Up => {
                        maybe_enqueue(p.mov(Dir::Left), Dir::Left, &mut q, &mut visited);
                        maybe_enqueue(p.mov(Dir::Right), Dir::Right, &mut q, &mut visited);
                    }
                };
            }
            Some(Some(c)) => panic!("unexpected character {:?}", c),
            _ => (),
        };
    }

    for l in map.iter() {
        for c in l.iter() {
            print!("{}", c);
        }
        println!();
    }
    println!();
    println!();
    let visited_set: BTreeSet<Point> = visited.iter().map(|(p, _d)| p).cloned().collect();
    let mut sum = 0;
    for (y, l) in map.iter().enumerate() {
        for (x, _c) in l.iter().enumerate() {
            if visited_set.contains(&Point(x, y)) {
                print!("#");
                sum+=1;
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("{}", sum);
    Ok(())
}
