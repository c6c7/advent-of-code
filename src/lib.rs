#![feature(iter_array_chunks)]

use {
    macros::make_init_parts,
    once_cell::sync::Lazy,
    std::{collections::HashMap, sync::Mutex},
};

pub static PARTS: Lazy<Parts> = Lazy::new(Parts::new);

make_init_parts!((2022, 1..=11));

type PartFn = fn(&str);
type Year = usize;
type Day = u8;

pub struct Parts {
    inner: Mutex<std::collections::HashMap<(Year, Day), [PartFn; 2]>>,
}
impl Parts {
    fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }
    pub fn get(&self, &(year, day): &(usize, u8)) -> [fn(&str); 2] {
        *self
            .inner
            .lock()
            .unwrap()
            .get(&(year, day))
            .unwrap_or_else(|| panic!("Unknown Year {year}, Day {day}"))
    }

    pub fn register(&self, (year, day): (usize, u8), parts: [fn(&str); 2]) {
        assert!(
            self.inner
                .lock()
                .unwrap()
                .insert((year, day), parts)
                .is_none(),
            "Duplicate Year {year}, Day {day} registered!"
        );
    }
}

pub fn split_whitespace_and_convert_to_i64(input: &str) -> impl Iterator<Item = i64> + '_ {
    input
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("Could not convert str to i64."))
}
