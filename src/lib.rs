
pub fn noop(_inp: String) {}

pub type DayFn = fn(String);

pub fn get_day(year: u32, day: u32) -> (DayFn, DayFn) {
    return match (year, day) {
        _ => {
            println!("Unknown day: {}", day);
            return (noop, noop);
        },
    };
}
