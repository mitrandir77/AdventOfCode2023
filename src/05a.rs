// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
#![feature(btree_cursors)]

use anyhow::Result;
#[macro_use]
extern crate scan_rules;
use scan_rules::scan;
use scan_rules::scanner::Word;

use std::collections::BTreeMap;
use std::io::BufRead;
use std::ops::Bound;


#[derive(Debug,PartialOrd, Ord, PartialEq, Eq)]
struct MapValue {
    dest: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    values: BTreeMap<u64, MapValue>,
}

impl Map {
    fn new() -> Self {
        Map {
            from: "".to_string(),
            to: "".to_string(),
            values: BTreeMap::new(),
        }
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let seed_line =  stdin.lock().lines().next().unwrap()?;
    let mut seed_iter = seed_line.split_whitespace();
    let mut maps = vec![];
    assert_eq!(seed_iter.next().unwrap(), "seeds:");
    let mut seeds: Vec<u64> = seed_iter
            .map(|n| n.parse().unwrap())
            .collect();

    let mut map = Map::new();
    for line in stdin.lock().lines().skip(1) {
        let line = line?;
        scan!(&line;
            (let from: Word, "-to-", let to: Word, " map:") => {
                map.from = from.to_string();
                map.to= to.to_string();
            },
            (let dest: u64, " ", let from: u64, " ", let length: u64) => {
                map.values.insert(from, MapValue {
                    dest,
                    length,
                });
            },
            ("") => {
                maps.push(map);
                map = Map::new();
            }
        )
        .unwrap();
    }
    maps.push(map);


    for map in maps {
        seeds = seeds.into_iter().map(
            |seed| {
                if let Some((key,MapValue {
                    dest, length
                })) = map.values.upper_bound(Bound::Included(&seed)).key_value() {
                    if seed < key + length {
                        return seed + dest - key;
                    }
                }
                seed
            }
        ).collect();
    }
    seeds.sort();
    println!("{}", seeds.first().unwrap());

    Ok(())
}
