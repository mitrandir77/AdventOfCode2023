// Advent of Code 2023
// (c) 2023 Mateusz Kwapich
use anyhow::Result;
use itertools::Itertools;
use std::io::Read;

#[macro_use]
extern crate scan_rules;
use scan_rules::{scan, scanner::re_str, scanner::Number};

fn hash(input: &str) -> u32 {
    let mut val = 0;
    for c in input.chars() {
        val += c as u32;
        val *= 17;
        val %= 256;
    }
    val
}

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let input = input.trim().split(',').collect_vec();

    let mut boxes: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];

    for op in input {
        scan!(op;
            (let label <| re_str(r"[a-z]+"), "-") => {
                let box_num = hash(label);
                boxes[box_num as usize].retain(|(el_label, _el_fnum)| el_label != label);
            },
            (let label <| re_str(r"[a-z]+"), "=", let fnum: Number) => {
                let box_num = hash(label);
                let label = label.to_owned();
                let fnum = fnum.parse().unwrap();
                match boxes[box_num as usize].iter().position(|(el_label, _el_fnum)|el_label == &label) {
                    Some(index) => {
                        boxes[box_num as usize][index] = (label, fnum);
                    },
                    None => {
                        boxes[box_num as usize].push((label, fnum))
                    }

                }

            }
        )
        .unwrap();
    }

    let mut sum = 0;
    for (box_num, lenses) in boxes.into_iter().enumerate() {
        for (slot, (_label, fnum)) in lenses.into_iter().enumerate() {
            sum += (box_num + 1) * (slot + 1) * fnum;
        }
    }

    println!("{sum}");
    Ok(())
}
