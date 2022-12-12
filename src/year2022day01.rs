use std::{
    cmp::{Ord, Ordering},
    collections::BinaryHeap,
};

#[derive(Default, Debug, Eq)]
struct Elf {
    total: i64,
    snacks: Vec<i64>,
}

impl Elf {
    fn add_snack(&mut self, snack: i64) {
        self.total += snack;
        self.snacks.push(snack);
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total.cmp(&other.total)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total
    }
}

fn build_elf_heap(input: String) -> BinaryHeap<Elf> {
    let mut elves = BinaryHeap::new();

    let mut elf = Elf::default();
    for snack in input.split('\n') {
        if snack.is_empty() {
            elves.push(elf);
            elf = Elf::default();
            continue;
        }
        elf.add_snack(snack.parse::<i64>().expect("Failed to read calorie count"));
    }
    elves
}

pub fn part1(input: &str) {
    let elves = build_elf_heap(input.to_string());
    println!("Number of elves: {}", elves.len());
    println!("Part 1: {:?}", elves.peek());
}

pub fn part2(input: &str) {
    let mut elves = build_elf_heap(input.to_string());
    let mut top3_total = 0;
    for _ in 0..3 {
        top3_total += elves.pop().unwrap().total;
    }
    println!("Top 3 Total: {top3_total}");
}
