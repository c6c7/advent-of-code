#![allow(clippy::all)]

const NEW_LANTERNFISH_INTERNAL_TIMER: usize = 9;
const PART_1_SIMULATION_LENGTH: usize = 80; // days
const PART_2_SIMULATION_LENGTH: usize = 256; // days

pub fn part1(input: String) {
    simulate_lanternfish(&input, PART_1_SIMULATION_LENGTH);
}

pub fn part2(input: String) {
    simulate_lanternfish(&input, PART_2_SIMULATION_LENGTH);
}

fn simulate_lanternfish(input: &str, simulation_length: usize) {
    let mut lanternfish = parse_lanternfish(&input);
    let mut zero = 0;
    for _ in 0..simulation_length {
        let new_lanternfish = lanternfish[zero];
        zero = (zero + 1) % NEW_LANTERNFISH_INTERNAL_TIMER;
        lanternfish
            [(zero + NEW_LANTERNFISH_INTERNAL_TIMER - 3) % NEW_LANTERNFISH_INTERNAL_TIMER] +=
            new_lanternfish;
    }
    println!("lanternfish: {:?}", lanternfish);
    println!(
        "after {} days: {}",
        simulation_length,
        lanternfish.iter().fold(0, |acc, f| acc + f)
    );
}

fn parse_lanternfish(input: &str) -> [u64; NEW_LANTERNFISH_INTERNAL_TIMER] {
    let mut lanternfish = [0u64; NEW_LANTERNFISH_INTERNAL_TIMER];
    input
        .trim()
        .split(",")
        .for_each(|lf| lanternfish[lf.parse::<usize>().unwrap()] += 1);
    lanternfish
}
