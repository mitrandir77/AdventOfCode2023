// Advent of Code 2022
// (c) 2002 Mateusz Kwapich
#![feature(pattern)]

use anyhow::Result;
use std::io::BufRead;

fn is_symbol(c: char) -> bool {
    if c.is_alphanumeric() {
        return false;
    }
    if c.is_whitespace() {
        return false;
    }
    if c == '.' {
        return false;
    }
    true
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut map = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        map.push(format!(".{line}."));
    }

    let mut sum = 0;
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
                let above = if y > 0 {
                    map.get(y - 1)
                        .unwrap_or(&"".to_owned())
                        .get(cur_num_x- 1..=x)
                        .unwrap_or("")
                        .contains(is_symbol)
                } else {
                    false
                };
                let below = map
                    .get(y + 1)
                    .unwrap_or(&"".to_owned())
                    .get(cur_num_x - 1..=x)
                    .unwrap_or("")
                    .contains(is_symbol);

                if above
                    || below
                    || is_symbol(line.chars().nth(x).unwrap())
                    || is_symbol(line.chars().nth(cur_num_x - 1).unwrap())
                {
                    sum += cur_num;
                }
                cur_num = 0;
                m_cur_num_x = None;
            }
        }
    }

    println!("{sum}");
    Ok(())
}
