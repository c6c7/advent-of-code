pub mod year2021;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(year: u32, day: u32) -> (DayFn, DayFn) {
    return match (year, day) {
        (2021, 1) => (year2021::day01::part1, noop),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        }
    };
}

pub fn split_whitespace_and_convert_to_i64<'a>(
    input: &'a String,
) -> Box<dyn Iterator<Item = i64> + 'a> {
    Box::new(
        input
            .split_whitespace()
            .map(|s| s.parse::<i64>().expect("Could not convert str to i64.")),
    )
}
