#![feature(iter_array_chunks)]

pub mod year2021;
pub mod year2022;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(year: u32, day: u32) -> (DayFn, DayFn) {
    return match (year, day) {
        (2021, 1) => (year2021::day01::part1, year2021::day01::part2),
        (2021, 2) => (year2021::day02::part1, year2021::day02::part2),
        (2021, 3) => (year2021::day03::part1, year2021::day03::part2),
        (2021, 4) => (year2021::day04::part1, year2021::day04::part2),
        // Forgot to split day 5 into two parts
        (2021, 5) => (year2021::day05::part1, noop),
        (2021, 6) => (year2021::day06::part1, year2021::day06::part2),
        (2021, 7) => (year2021::day07::part1, year2021::day07::part2),
        (2021, 8) => (year2021::day08::part1, year2021::day08::part2),
        (2021, 9) => (year2021::day09::part1, year2021::day09::part2),
        (2021, 10) => (year2021::day10::part1, year2021::day10::part2),
        (2021, 11) => (year2021::day11::part1, noop),
        (2021, 12) => (year2021::day12::part1, year2021::day12::part2),
        (2021, 13) => (year2021::day13::part1, noop),
        (2021, 14) => (year2021::day14::part1, noop),
        (2022, 1) => (year2022::day01::part1, year2022::day01::part2),
        (2022, 2) => (year2022::day02::part1, year2022::day02::part2),
        (2022, 3) => (year2022::day03::part1, year2022::day03::part2),
        (2022, 4) => (year2022::day04::part1, year2022::day04::part2),
        (2022, 5) => (year2022::day05::part1, year2022::day05::part2),
        (2022, 6) => (year2022::day06::part1, year2022::day06::part2),
        (2022, 7) => (year2022::day07::part1, year2022::day07::part2),
        (2022, 8) => (year2022::day08::part1, year2022::day08::part2),
        _ => {
            panic!("Unknown day: {}", day);
        }
    };
}

pub fn split_whitespace_and_convert_to_i64(input: &str) -> impl Iterator<Item = i64> + '_ {
    input
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("Could not convert str to i64."))
}
