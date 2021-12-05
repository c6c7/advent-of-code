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

pub fn part1(input: String) {
    let input = input
        .split_whitespace()
        .map(|s| u16::from_str_radix(s, 2).expect("Could not convert binary to i64."));
    let gamma_rate = calc_gamma_rate(input);
    let epsilon_rate = calc_epsilon_rate(gamma_rate);

    println!("Epsilon Rate: {:012b}", epsilon_rate);
    println!("Gamma Rate: {:012b}", gamma_rate);
    println!(
        "Puzzle Answer: {}",
        (epsilon_rate as u64) * (gamma_rate as u64)
    );
}
