use std::collections::HashMap;

pub fn part1(input: String) {
    let mut parts = input.split("\n\n");
    let polymer = parts.next().unwrap().trim().chars().collect::<Vec<char>>();
    let pair_insertion_rules = parts
        .next()
        .unwrap()
        .trim()
        .split("\n")
        .map(|rule_s| parse_pair_insertion_rule(rule_s))
        .collect();

    let occurrences =
        apply_pair_insertion_rules(&polymer[..], &pair_insertion_rules, 40, &mut HashMap::new());
    println!("Occurrences: {:?}", occurrences);

    let mut max = 0;
    let mut min = usize::MAX;
    for k in occurrences.into_values() {
        if k > max {
            max = k;
        }
        if k < min {
            min = k;
        }
    }
    println!("Answer: {}", max - min);
}

#[allow(dead_code)]
fn format_polymer(polymer: &[char]) -> String {
    polymer
        .iter()
        .fold(String::new(), |acc, c| format!("{}{}", acc, c))
}

struct PairInsertionRule {
    pub predicate: (char, char),
    pub insertion: char,
}

fn parse_pair_insertion_rule(rule_s: &str) -> PairInsertionRule {
    let mut parts = rule_s.split_whitespace();
    let mut predicate_parts = parts.next().unwrap().chars();
    let predicate = (
        predicate_parts.next().unwrap(),
        predicate_parts.next().unwrap(),
    );
    parts.next();
    let insertion = parts.next().unwrap().chars().next().unwrap();
    PairInsertionRule {
        predicate,
        insertion,
    }
}

fn format_key(polymer: &[char], steps: usize) -> String {
    format!("{}{}", format_polymer(polymer), steps)
}

fn apply_pair_insertion_rules(
    polymer: &[char],
    pair_insertion_rules: &Vec<PairInsertionRule>,
    steps: usize,
    pair_insertion_lookup: &mut HashMap<String, HashMap<char, usize>>,
) -> HashMap<char, usize> {
    let key = format_key(polymer, steps);
    match pair_insertion_lookup.get(&key).map(|entry| entry.clone()) {
        Some(result) => result,
        None => {
            if steps == 0 || polymer.len() < 2 {
                let mut result = HashMap::new();
                for c in polymer {
                    *result.entry(*c).or_insert(0) += 1;
                }
                return result;
            }

            if polymer.len() == 2 {
                let mut new_polymer = vec![polymer[0]];
                for rule in pair_insertion_rules {
                    if polymer[0] == rule.predicate.0 && polymer[1] == rule.predicate.1 {
                        new_polymer.push(rule.insertion);
                        break;
                    }
                }
                new_polymer.push(polymer[1]);
                return apply_pair_insertion_rules(
                    &new_polymer[..],
                    pair_insertion_rules,
                    steps - 1,
                    pair_insertion_lookup,
                );
            }

            // Recursively apply pair insertion to each part of the polymer
            let mut one = apply_pair_insertion_rules(
                &polymer[..polymer.len() / 2],
                pair_insertion_rules,
                steps,
                pair_insertion_lookup,
            );
            let mut two = apply_pair_insertion_rules(
                &polymer[polymer.len() / 2 - 1..polymer.len() / 2 + 1],
                pair_insertion_rules,
                steps,
                pair_insertion_lookup,
            );
            let mut three = apply_pair_insertion_rules(
                &polymer[polymer.len() / 2..],
                pair_insertion_rules,
                steps,
                pair_insertion_lookup,
            );

            // Merge the steps
            for (k, v) in one.iter_mut() {
                *v += two.remove(&k).unwrap_or(0);
                *v += three.remove(&k).unwrap_or(0);
            }
            for (k, v) in two.iter() {
                one.insert(*k, *v);
            }
            for (k, v) in three.iter() {
                *one.entry(*k).or_insert(0) += v;
            }

            *one.get_mut(&polymer[polymer.len() / 2 - 1]).unwrap() -= 1;
            match one.get_mut(&polymer[polymer.len() / 2]) {
                Some(v) => *v -= 1,
                None => {
                    println!("polymer: {:?}", polymer);
                    println!("one: {:?}", one);
                    println!("two: {:?}", two);
                    println!("three: {:?}", three);
                    panic!(
                        "one does not contain {}: steps {:?}, polymer {:?}",
                        polymer[polymer.len() / 2],
                        steps,
                        polymer
                    );
                }
            }

            pair_insertion_lookup.insert(key, one.clone());
            one
        }
    }
}
