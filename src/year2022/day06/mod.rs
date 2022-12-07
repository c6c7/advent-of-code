use tracing::debug;

fn insert_byte(map: &mut std::collections::HashMap<u8, usize>, b: &u8) {
    match map.get_mut(b) {
        None => {
            map.insert(*b, 1);
        }
        Some(count) => {
            *count += 1;
        }
    }
}

fn remove_byte(map: &mut std::collections::HashMap<u8, usize>, b: &u8) {
    match map.get_mut(b) {
        None => unreachable!(),
        Some(count) => {
            if *count == 1 {
                map.remove(b);
            } else {
                *count -= 1;
            }
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct FirstMarker {
    characters: String,
    position: usize,
}

fn start_of_packet_marker(input: &str) -> FirstMarker {
    start_marker(input, 4)
}

fn start_of_message_marker(input: &str) -> FirstMarker {
    start_marker(input, 14)
}

fn start_marker(input: &str, distinct_count: usize) -> FirstMarker {
    let input_bytes = input.as_bytes();
    let mut last_four_map = {
        let mut map = std::collections::HashMap::new();
        for b in &input_bytes[..distinct_count] {
            insert_byte(&mut map, b);
        }
        map
    };
    let mut last_four_stack: std::collections::VecDeque<_> =
        input_bytes[..distinct_count].to_vec().into();
    let mut position = distinct_count;
    for b in &input_bytes[distinct_count..] {
        debug!("last_four_map: {last_four_map:?}");
        if last_four_map.len() == distinct_count {
            return FirstMarker {
                characters: String::from_utf8(last_four_stack.into()).unwrap(),
                position,
            };
        }
        remove_byte(&mut last_four_map, &last_four_stack.pop_front().unwrap());
        insert_byte(&mut last_four_map, b);
        last_four_stack.push_back(*b);
        position += 1;
    }
    unreachable!();
}

pub fn part1(input: String) {
    tracing::info!("Part 1 Answer: {:?}", start_of_packet_marker(&input));
}

pub fn part2(input: String) {
    tracing::info!("Part 2 Answer: {:?}", start_of_message_marker(&input));
}

#[cfg(test)]
mod tests {
    use {super::*, test_case::test_case, tracing_test::traced_test};

    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    #[traced_test]
    fn first_packet_marker_location(input: &str, expected_position: usize) {
        assert_eq!(start_of_packet_marker(input).position, expected_position);
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    #[traced_test]
    fn first_message_marker_location(input: &str, expected_position: usize) {
        assert_eq!(start_of_message_marker(input).position, expected_position);
    }
}
