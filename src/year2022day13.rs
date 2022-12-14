use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum PacketValue {
    Packet(Packet),
    Value(u8),
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (PacketValue::Value(u), PacketValue::Value(v)) => Some(u.cmp(v)),
            (PacketValue::Packet(p), PacketValue::Packet(q)) => p.partial_cmp(q),
            (PacketValue::Value(v), p @ PacketValue::Packet(..)) => {
                PacketValue::Packet(Packet(vec![PacketValue::Value(*v)])).partial_cmp(p)
            }
            (p @ PacketValue::Packet(..), PacketValue::Value(v)) => {
                p.partial_cmp(&PacketValue::Packet(Packet(vec![PacketValue::Value(*v)])))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Packet(Vec<PacketValue>);

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in 0..std::cmp::min(self.0.len(), other.0.len()) {
            match self.0[i].partial_cmp(&other.0[i]) {
                Some(Ordering::Equal) => continue,
                o => return o,
            }
        }
        Some(self.0.len().cmp(&other.0.len()))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Packet {
    fn from_str(s: &str) -> Self {
        if s.is_empty() {
            panic!("Cannot create Packet from empty string");
        }
        let mut stack = vec![];
        let mut tmp_value = None;
        for b in s.as_bytes() {
            match b {
                b'[' => {
                    stack.push(vec![]);
                }
                b',' => {
                    if let Some(value) = tmp_value.take() {
                        stack.last_mut().unwrap().push(PacketValue::Value(value))
                    }
                    continue;
                }
                b']' => {
                    if let Some(value) = tmp_value.take() {
                        stack.last_mut().unwrap().push(PacketValue::Value(value))
                    }
                    let complete_packet = stack.pop().unwrap();
                    if stack.is_empty() {
                        return Packet(complete_packet);
                    }
                    stack
                        .last_mut()
                        .unwrap()
                        .push(PacketValue::Packet(Packet(complete_packet)));
                }
                value => {
                    let value = *value - b'0';
                    tmp_value = match tmp_value {
                        None => Some(value),
                        Some(a) => Some(10 * a + value),
                    };
                }
            }
        }
        unreachable!()
    }
}

struct PacketPair(Packet, Packet, String);

fn get_packet_pairs(input: &str) -> Vec<PacketPair> {
    input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let mut pair_iter = pair.split('\n');
            PacketPair(
                Packet::from_str(pair_iter.next().unwrap()),
                Packet::from_str(pair_iter.next().unwrap()),
                pair.to_string(),
            )
        })
        .collect()
}

pub fn part1(input: &str) {
    let packet_pairs = get_packet_pairs(input);
    let mut ans = 0;
    for (idx, packet_pair) in packet_pairs.iter().enumerate().map(|(i, p)| (i + 1, p)) {
        let o = packet_pair.0.partial_cmp(&packet_pair.1).unwrap();
        tracing::debug!("\n{}\n{o:?}", packet_pair.2);
        match o {
            Ordering::Less | Ordering::Equal => {
                ans += idx;
            }
            _ => (),
        }
    }
    tracing::info!("Part 1 Answer: {ans}");
}

pub fn part2(input: &str) {
    let mut packets = get_packet_pairs(input).into_iter().fold(
        vec![Packet::from_str("[[2]]"), Packet::from_str("[[6]]")],
        |mut acc, PacketPair(p, q, _)| {
            acc.push(p);
            acc.push(q);
            acc
        },
    );
    packets.sort();

    tracing::info!(
        "Part 2 Answer: {}",
        packets
            .iter()
            .enumerate()
            .filter(|(_, p)| **p == Packet::from_str("[[2]]") || **p == Packet::from_str("[[6]]"))
            .map(|(i, _)| i)
            .fold(1, |acc, i| acc * (i + 1))
    )
}

#[cfg(test)]
mod tests {
    use {super::*, test_case::test_case, tracing_test::traced_test};

    #[traced_test]
    #[test_case("[]", Packet(vec![]); "empty")]
    #[test_case("[1]", Packet(vec![PacketValue::Value(1)]); "single value")]
    #[test_case("[1,[2]]", Packet(vec![PacketValue::Value(1), PacketValue::Packet(Packet(vec![PacketValue::Value(2)]))]); "mixed values")]
    fn packet_parsing(packet_str: &str, expected: Packet) {
        assert_eq!(Packet::from_str(packet_str), expected);
    }

    #[traced_test]
    #[test_case("[]", "[]", Ordering::Equal; "empty equal")]
    #[test_case("[1]", "[1]", Ordering::Equal; "single equal")]
    #[test_case("[10]", "[10]", Ordering::Equal; "double-digit equal")]
    #[test_case("[2]", "[12]", Ordering::Less; "mixed-digit less than")]
    #[test_case("[1]", "[2]", Ordering::Less; "single less than")]
    #[test_case("[3]", "[2]", Ordering::Greater; "single greater than")]
    #[test_case("[1,2]", "[1,3]", Ordering::Less; "multiple less than")]
    #[test_case("[1,3]", "[1,3]", Ordering::Equal; "multiple equal")]
    #[test_case("[1,3]", "[1,2]", Ordering::Greater; "multiple greater than")]
    #[test_case("[[1]]", "[1]", Ordering::Equal; "nested equal")]
    #[test_case("[1,2]", "[1]", Ordering::Greater; "first longer")]
    #[test_case("[[1,2]]", "[1]", Ordering::Greater; "nested first longer")]
    #[test_case("[1]", "[1,2]", Ordering::Less; "second longer")]
    #[test_case("[[[]]]", "[[]]", Ordering::Greater; "nested empty")]
    #[test_case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", Ordering::Greater; "complex example")]
    fn packet_comparison(a: &str, b: &str, expected: Ordering) {
        assert_eq!(
            Packet::from_str(a)
                .partial_cmp(&Packet::from_str(b))
                .unwrap(),
            expected
        );
    }
}
