// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
#![feature(pattern)]

use anyhow::Result;
use std::{collections::BTreeMap, io::BufRead};

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut map = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        map.push(format!(".{line}."));
    }

    let mut asterisks = BTreeMap::new();
    let mut ratios = BTreeMap::new();
    for (y, line) in map.iter().enumerate() {
        let mut cur_num = 0;
        let mut m_cur_num_x = None;
        for (x, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                cur_num *= 10;
                cur_num += digit;
                if m_cur_num_x.is_none() {
                    m_cur_num_x = Some(x);
                }
            } else if let Some(cur_num_x) = m_cur_num_x {
                // finished parsing number
                if y > 0 {
                    for (pos, _) in map
                        .get(y - 1)
                        .unwrap_or(&"".to_owned())
                        .get(cur_num_x - 1..=x)
                        .unwrap_or("")
                        .match_indices('*')
                    {
                        *asterisks.entry((cur_num_x - 1 + pos, y - 1)).or_insert(0) += 1;
                        *ratios.entry((cur_num_x - 1 + pos, y - 1)).or_insert(1) *= cur_num;
                    }
                };
                for (pos, _) in map
                    .get(y + 1)
                    .unwrap_or(&"".to_owned())
                    .get(cur_num_x - 1..=x)
                    .unwrap_or("")
                    .match_indices('*')
                {
                    *asterisks.entry((cur_num_x - 1 + pos, y + 1)).or_insert(0) += 1;
                    *ratios.entry((cur_num_x - 1 + pos, y + 1)).or_insert(1) *= cur_num;
                }

                if line.chars().nth(x).unwrap() == '*' {
                    *asterisks.entry((x, y)).or_insert(0) += 1;
                    *ratios.entry((x, y)).or_insert(1) *= cur_num;
                }
                if line.chars().nth(cur_num_x - 1).unwrap() == '*' {
                    *asterisks.entry((cur_num_x - 1, y)).or_insert(0) += 1;
                    *ratios.entry((cur_num_x - 1, y)).or_insert(1) *= cur_num;
                }
                cur_num = 0;
                m_cur_num_x = None;
            }
        }
    }

    let mut sum = 0;
    for (key, num) in asterisks.iter() {
        if *num == 2 {
            sum += ratios.get(key).unwrap();
        }
    }

    println!("{sum}");
    Ok(())
}
