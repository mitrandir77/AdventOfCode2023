// Advent of Code 2023
// (c) 2023 Mateusz Kwapich

use anyhow::Result;
use itertools::Itertools;
use std::cmp::Ordering;
use std::io::BufRead;

fn hand_type(hand: Vec<u32>) -> u32 {
    let mut counts = hand.into_iter().counts();
    let jokers = counts.remove(&0).unwrap_or(0);
    let val = counts.values().cloned().sorted().rev().collect_vec();

    let first = *val.first().unwrap_or(&0);
    let second = *val.get(1).unwrap_or(&0);
    if first + jokers == 5 {
        6
    } else if first + jokers == 4 {
        5
    } else if first + jokers == 3 && second == 2 {
        4
    } else if first + jokers == 3 {
        3
    } else if first == 2 && second == 2 {
        2
    } else if first + jokers == 2 {
        1
    } else {
        0
    }
}

fn compare_hands((a, _a_bid): &(Vec<u32>, u64), (b, _b_bid): &(Vec<u32>, u64)) -> Ordering {
    let a_type = hand_type(a.clone());
    let b_type = hand_type(b.clone());
    if a_type != b_type {
        a_type.cmp(&b_type)
    } else {
        a.cmp(b)
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();

    let mut winnings: u64 = 0;
    let mut hands = vec![];
    for line in stdin.lock().lines() {
        let line = line?;
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid: u64 = bid.parse().unwrap();
        let hand = hand
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 0,
                'T' => 10,
                d if d.is_ascii_digit() => d.to_digit(10).unwrap(),
                _ => panic!("unknown hand char"),
            })
            .collect_vec();
        hands.push((hand, bid));
    }
    hands.sort_by(compare_hands);
    for (rank, (_hand, bid)) in hands.into_iter().enumerate() {
        let rank = (rank + 1) as u64;
        winnings = winnings.checked_add(rank * bid).unwrap();
    }

    println!("{winnings}");
    Ok(())
}
