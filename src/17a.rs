// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::io::BufRead;
use std::iter;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turns(&self) -> [Self; 2] {
        match self {
            Self::Up => [Self::Left, Self::Right],
            Self::Down => [Self::Left, Self::Right],
            Self::Left => [Self::Up, Self::Down],
            Self::Right => [Self::Up, Self::Down],
        }
    }
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

// dist, position, direction, steps_since_dir_change
#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchState(usize, Point, Dir, usize);

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(start: Point, dest: Point, map: &Vec<Vec<usize>>) -> usize {
    let mut q: BinaryHeap<SearchState> = BinaryHeap::new();
    let mut seen = BTreeMap::new();

    for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
        if let Some(p) = start.mov(dir) {
            if let Some(Some(heat_loss)) = map.get(p.1).map(|l| l.get(p.0)) {
                q.push(SearchState(*heat_loss, p, dir, 1));
                seen.insert((p, dir, 1), *heat_loss);
            }
        }
    }

    while let Some(SearchState(dist, p, dir, steps_since_dir_change)) = q.pop() {
        let cont: Box<dyn Iterator<Item = (Dir, usize)>> = if steps_since_dir_change < 3 {
            Box::new(iter::once((dir, steps_since_dir_change + 1)))
        } else {
            Box::new(iter::empty())
        };
        for (dir, steps_since_dir_change) in
            dir.turns().into_iter().zip(iter::repeat(1)).chain(cont)
        {
            if let Some(p) = p.mov(dir) {
                if let Some(Some(heat_loss)) = map.get(p.1).map(|l| l.get(p.0)) {
                    if let std::collections::btree_map::Entry::Vacant(e) =
                        seen.entry((p, dir, steps_since_dir_change))
                    {
                        let dist = dist + heat_loss;
                        e.insert(dist);
                        q.push(SearchState(dist, p, dir, steps_since_dir_change));
                    }
                }
            }
        }
    }
    let mut min = usize::MAX;
    for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
        for steps_since_dir_change in 1..=3 {
            if let Some(dist) = seen.get(&(dest, dir, steps_since_dir_change)) {
                min = min.min(*dist);
            }
        }
    }
    min
}

fn main() -> Result<()> {
    let mut map: Vec<Vec<usize>> = vec![];
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        map.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }

    let min = shortest_path(Point(0, 0), Point(map[0].len() - 1, map.len() - 1), &map);
    println!("{}", min);
    Ok(())
}
