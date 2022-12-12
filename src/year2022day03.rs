use {
    std::convert::TryInto,
    tracing::{debug, info},
};

fn priority(mut item: char) -> usize {
    const P0: u8 = b'a' - 1;

    assert!(item.is_ascii());
    let mut bias = 0;
    if item.is_uppercase() {
        bias += 26;
        item = item.to_ascii_lowercase();
    }

    (bias + <char as std::convert::TryInto<u8>>::try_into(item).unwrap() - P0)
        .try_into()
        .unwrap()
}

fn find_duplicate(rucksack: &str) -> char {
    let mut c1 = std::collections::HashSet::new();
    let mut rucksack_items = rucksack.chars();
    for _ in 0..rucksack.len() / 2 {
        c1.insert(rucksack_items.next().unwrap());
    }
    for _ in rucksack.len() / 2..rucksack.len() {
        let item = rucksack_items.next().unwrap();
        if c1.contains(&item) {
            return item;
        }
    }
    unreachable!()
}

pub fn part1(input: &str) {
    let rucksack_list = input.trim().split('\n');
    let mut sum: usize = 0;
    for rucksack in rucksack_list {
        debug!("--- new rucksack ---");
        debug!("duplicate: {}", find_duplicate(rucksack));
        rucksack.chars().for_each(|c| {
            debug!("{}: {}", c, priority(c));
        });
        sum += priority(find_duplicate(rucksack));
    }
    info!("Part 1 Answer: {}", sum);
}

pub fn part2(input: &str) {
    let rucksack_list = input.trim().split('\n');
    let mut sum = 0;
    for group in rucksack_list.array_chunks::<3>() {
        let mut rucksacks_with_item: [u8; 53] = [0; 53];
        let mut indicators = std::collections::HashSet::new();
        for rucksack in group {
            indicators.clear();
            debug!("rucksack: {:?}", rucksack);
            for p in rucksack.chars().map(priority) {
                indicators.insert(p);
            }
            debug!("indicators: {:?}", indicators);

            // Accumulate indicators
            for p in &indicators {
                rucksacks_with_item[*p] += 1;
            }
            debug!("rucksacks_with_item: {:?}", rucksacks_with_item);
        }

        // Accumulate priority for item appearing in three rucksacks
        for (p, item) in rucksacks_with_item.iter().skip(1).enumerate() {
            if *item == 3 {
                sum += p + 1;
                break;
            }
        }
    }
    info!("Part 2 Answer: {}", sum);
}
