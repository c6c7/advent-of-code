// Days
pub mod year2015;

pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(year: u32, day: u32) -> (DayFn, DayFn) {
    return match (year, day) {
        (2015, 1) => (year2015::day01::part1, noop),
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        },
    };
}
