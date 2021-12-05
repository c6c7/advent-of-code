pub mod year2021;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(year: u32, day: u32) -> (DayFn, DayFn) {
    return match (year, day) {
        (2021, 1) => (year2021::day01::part1, year2021::day01::part2),
        (2021, 2) => (year2021::day02::part1, year2021::day02::part2),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}

pub fn split_whitespace_and_convert_to_i64<'a>(
    input: &'a String,
) -> impl Iterator<Item = i64> + 'a {
    input
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("Could not convert str to i64."))
}
