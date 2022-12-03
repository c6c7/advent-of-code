#[rustfmt::skip]
use std::{convert::TryInto};

fn priority(mut item: char) -> u8 {
    const P0: u8 = b'a' - 1;

    assert!(item.is_ascii());
    let mut bias = 0;
    if item.is_uppercase() {
        bias += 26;
        item = item.to_ascii_lowercase();
    }

    bias + <char as TryInto<u8>>::try_into(item).unwrap() - P0
}

pub fn part1(input: String) {
    let rucksack_list = input.split("\n");
    for rucksack in rucksack_list {
        println!("--- new rucksack ---");
        rucksack.chars().for_each(|c| {
            println!("{}: {}", c, priority(c));
        })
    }
}

pub fn part2(_input: String) {}
