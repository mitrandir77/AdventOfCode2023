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

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct MapValue {
    dest: i64,
    length: i64,
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    values: BTreeMap<i64, MapValue>,
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

    let seed_line = stdin.lock().lines().next().unwrap()?;
    let mut seed_iter = seed_line.split_whitespace();
    let mut maps = vec![];
    assert_eq!(seed_iter.next().unwrap(), "seeds:");
    let seeds: Vec<i64> = seed_iter.map(|n| n.parse::<i64>().unwrap()).collect();
    let mut seeds: Vec<(i64, i64)> = seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();

    let mut map = Map::new();
    for line in stdin.lock().lines().skip(1) {
        let line = line?;
        scan!(&line;
            (let from: Word, "-to-", let to: Word, " map:") => {
                map.from = from.to_string();
                map.to= to.to_string();
            },
            (let dest: i64, " ", let from: i64, " ", let length: i64) => {
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
        seeds = seeds
            .into_iter()
            .flat_map(|(mut seed_start, mut seed_length)| {
                let mut res = vec![];

                loop {
                    if let Some((key, MapValue { dest, length })) = map
                        .values
                        .upper_bound(Bound::Included(&seed_start))
                        .key_value()
                    {
                        if seed_start < key + length {
                            if seed_start + seed_length <= *key + *length {
                                res.push((seed_start + dest - key, seed_length));
                                return res;
                            } else {
                                res.push((seed_start + dest - key, length + key - seed_start));
                                seed_length -= length + key - seed_start;
                                seed_start += length + key - seed_start;
                                continue;
                            }
                        }
                    }

                    if let Some((key, MapValue { dest: _, length: _ })) = map
                        .values
                        .lower_bound(Bound::Excluded(&seed_start))
                        .key_value()
                    {
                        if seed_start + seed_length > *key {
                            res.push((seed_start, *key - seed_start));
                            seed_length -= *key - seed_start;
                            seed_start = *key;
                            continue;
                        }
                    }
                    res.push((seed_start, seed_length));
                    return res;
                }
            })
            .collect();
        // dbg!(&seeds);
    }
    let mut seeds: Vec<_> = seeds.into_iter().map(|(k, _v)| k).collect();
    seeds.sort();
    println!("{}", seeds.first().unwrap());

    Ok(())
}
