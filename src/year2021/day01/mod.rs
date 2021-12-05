use crate::split_whitespace_and_convert_to_i64;

pub fn part1(input: String) {
    let last = split_whitespace_and_convert_to_i64(&input);
    let cur = {
        let mut cur = split_whitespace_and_convert_to_i64(&input);
        cur.next();
        cur
    };

    println!(
        "Number increasing: {}",
        cur.zip(last).fold(0, |increasing_count, (cur, last)| {
            if cur > last {
                increasing_count + 1
            } else {
                increasing_count
            }
        })
    );
}
