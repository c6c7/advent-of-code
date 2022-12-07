use crate::split_whitespace_and_convert_to_i64;

fn number_increasing(last: impl Iterator<Item = i64>, cur: impl Iterator<Item = i64>) -> usize {
    cur.zip(last).fold(0, |increasing_count, (cur, last)| {
        if cur > last {
            increasing_count + 1
        } else {
            increasing_count
        }
    })
}

pub fn part1(input: String) {
    let last = split_whitespace_and_convert_to_i64(&input);
    let cur = {
        let mut cur = split_whitespace_and_convert_to_i64(&input);
        cur.next();
        cur
    };

    println!("Number increasing: {}", number_increasing(last, cur),);
}

fn window_sums(input: &str) -> impl Iterator<Item = i64> + '_ {
    let depths0 = split_whitespace_and_convert_to_i64(input);
    let mut depths1 = split_whitespace_and_convert_to_i64(input);
    depths1.next();
    let depths1 = depths1;
    let mut depths2 = split_whitespace_and_convert_to_i64(input);
    depths2.next();
    depths2.next();
    let depths2 = depths2;

    depths0
        .zip(depths1)
        .zip(depths2)
        .map(|((a, b), c)| a + b + c)
}

pub fn part2(input: String) {
    let last = window_sums(&input);
    let cur = {
        let mut cur = window_sums(&input);
        cur.next();
        cur
    };
    println!("Number increasing: {}", number_increasing(last, cur));
}
