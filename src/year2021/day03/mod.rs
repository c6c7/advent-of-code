const NUM_BINARY_DIGITS: usize = 12;

fn calc_gamma_rate<'a>(input: impl Iterator<Item = u16> + 'a) -> u16 {
    let most_common_acc = input.fold([0i64; NUM_BINARY_DIGITS], |mut most_common_acc, number| {
        for i in 0..NUM_BINARY_DIGITS {
            match (number & (1 << i)) >> i {
                1 => most_common_acc[i] += 1,
                0 => most_common_acc[i] -= 1,
                _ => unreachable!(),
            }
        }
        most_common_acc
    });
    most_common_acc
        .iter()
        .enumerate()
        .fold(0, |mut gamma_rate, (i, d)| {
            if *d > 0 {
                gamma_rate |= 1 << i;
            }
            gamma_rate
        })
}

fn calc_epsilon_rate(gamma_rate: u16) -> u16 {
    !gamma_rate & !(u16::MAX << NUM_BINARY_DIGITS)
}

fn split_whitespace_and_convert_binary<'a>(input: &'a String) -> impl Iterator<Item = u16> + 'a {
    input
        .split_whitespace()
        .map(|s| u16::from_str_radix(s, 2).expect("Could not convert binary to i64."))
}

pub fn part1(input: String) {
    let input = split_whitespace_and_convert_binary(&input);
    let gamma_rate = calc_gamma_rate(input);
    let epsilon_rate = calc_epsilon_rate(gamma_rate);

    println!("Epsilon Rate: {:012b}", epsilon_rate);
    println!("Gamma Rate: {:012b}", gamma_rate);
    println!(
        "Puzzle Answer: {}",
        (epsilon_rate as u64) * (gamma_rate as u64)
    );
}

fn filter_by_preferred<F>(input: &String, preferred_fn: F) -> u16
where
    F: Fn(i64) -> u16,
{
    let mut remaining: Vec<u16> = split_whitespace_and_convert_binary(&input).collect();
    for i in 0..NUM_BINARY_DIGITS {
        let shift = NUM_BINARY_DIGITS - 1 - i;
        let preferred = preferred_fn(remaining.iter().fold(0, |acc, n| {
            if *n & (1 << shift) > 0 {
                acc + 1
            } else {
                acc - 1
            }
        }));
        assert!(preferred == 0 || preferred == 1);
        remaining = remaining
            .into_iter()
            .filter(|n| (*n & (1 << shift)) >> shift == preferred)
            .collect();
        if remaining.len() == 1 {
            break;
        }
    }
    remaining[0]
}

pub fn part2(input: String) {
    let oxygen_generator_rating = filter_by_preferred(&input, |acc| if acc >= 0 { 1 } else { 0 });
    let co2_scrubber_rating = filter_by_preferred(&input, |acc| if acc >= 0 { 0 } else { 1 });

    println!("Oxygen Generator Rating: {}", oxygen_generator_rating);
    println!("CO2 Scrubber Rating: {}", co2_scrubber_rating);
    println!(
        "Life Support Rating: {}",
        (oxygen_generator_rating as u64) * (co2_scrubber_rating as u64)
    );
}
