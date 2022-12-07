#![allow(clippy::all)]

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part1(input: String) {
    let unique_signal_patterns = input.split("\n").fold(0, |acc, l| {
        let mut parts = l.split("|");
        let _patterns = parts.next().unwrap().trim();
        let output_value = parts.next().unwrap().trim();
        acc + output_value.split_whitespace().fold(0, |acc, p| {
            if let Some(_) = [2, 4, 3, 7].iter().find(|&&x| p.len() == x) {
                acc + 1
            } else {
                acc
            }
        })
    });
    println!("Unique signal patterns: {}", unique_signal_patterns);
}

pub fn part2(input: String) {
    let answer = input.split("\n").fold(0, |acc, l| {
        println!("{}", l);
        let mut parts = l.split("|");
        let patterns = parts
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|p| HashSet::from_iter(p.chars()))
            .collect::<Vec<HashSet<char>>>();
        let (one, four, seven, eight, rest) = deduce_1_4_7_8(patterns);
        let (two, five, six, rest) = deduce_2_5_6(&one, rest);
        let (three, rest) = deduce_3(rest);
        let (zero, nine) = deduce_0_9(&four, rest);

        let digit_sets = vec![
            zero.0.clone(),
            one.0.clone(),
            two.0.clone(),
            three.0.clone(),
            four.0.clone(),
            five.0.clone(),
            six.0.clone(),
            seven.0.clone(),
            eight.0.clone(),
            nine.0.clone(),
        ];

        // println!("zero: {:?}", zero);
        // println!("one: {:?}", one);
        // println!("two: {:?}", two);
        // println!("three: {:?}", three);
        // println!("four: {:?}", four);
        // println!("five: {:?}", five);
        // println!("six: {:?}", six);
        // println!("seven: {:?}", seven);
        // println!("eight: {:?}", eight);
        // println!("nine: {:?}", nine);

        let output_value = parts
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .enumerate()
            .fold(0, |acc, (i, p)| {
                for d in 0..10 {
                    if digit_sets[d] == HashSet::from_iter(p.chars()) {
                        return acc + d * (10i64.pow((3 - i) as u32)) as usize;
                    }
                }
                unreachable!()
            });
        println!("output_value: {}", output_value);
        acc + output_value
    });
    println!("Answer: {}", answer);
}

#[derive(Debug)]
struct Zero(HashSet<char>);
#[derive(Debug)]
struct One(HashSet<char>);
#[derive(Debug)]
struct Two(HashSet<char>);
#[derive(Debug)]
struct Three(HashSet<char>);
#[derive(Debug)]
struct Four(HashSet<char>);
#[derive(Debug)]
struct Five(HashSet<char>);
#[derive(Debug)]
struct Six(HashSet<char>);
#[derive(Debug)]
struct Seven(HashSet<char>);
#[derive(Debug)]
struct Eight(HashSet<char>);
#[derive(Debug)]
struct Nine(HashSet<char>);

fn deduce_1_4_7_8(patterns: Vec<HashSet<char>>) -> (One, Four, Seven, Eight, Vec<HashSet<char>>) {
    let rest = patterns;
    let (mut one, rest): (Vec<HashSet<char>>, _) = rest.into_iter().partition(|p| p.len() == 2);
    let one = One(one.remove(0));
    let (mut four, rest): (Vec<HashSet<char>>, _) = rest.into_iter().partition(|p| p.len() == 4);
    let four = Four(four.remove(0));
    let (mut seven, rest): (Vec<HashSet<char>>, _) = rest.into_iter().partition(|p| p.len() == 3);
    let seven = Seven(seven.remove(0));
    let (mut eight, rest): (Vec<HashSet<char>>, _) = rest.into_iter().partition(|p| p.len() == 7);
    let eight = Eight(eight.remove(0));
    (one, four, seven, eight, rest)
}

fn deduce_2_5_6(one: &One, patterns: Vec<HashSet<char>>) -> (Two, Five, Six, Vec<HashSet<char>>) {
    let rest = patterns;

    let (mut candidates, rest): (Vec<HashSet<char>>, _) =
        rest.into_iter().partition(|p| !one.0.is_subset(p));

    let mut two = None;
    let mut five = None;
    let mut six = None;

    let seg = one.0.iter().next().unwrap();
    let indices = candidates
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (i, c)| {
            if c.contains(seg) {
                acc.push(i)
            }
            acc
        });
    match indices.len() {
        2 => {
            let first = candidates.remove(indices[0]);
            let second = candidates.remove(indices[1] - 1);
            if first.len() > second.len() {
                six.replace(first);
                five.replace(second);
            } else {
                five.replace(first);
                six.replace(second);
            }
            two.replace(candidates.remove(0));
        }
        1 => {
            two.replace(candidates.remove(indices[0]));
            let first = candidates.remove(0);
            let second = candidates.remove(0);
            if first.len() > second.len() {
                six.replace(first);
                five.replace(second);
            } else {
                five.replace(first);
                six.replace(second);
            }
        }
        _ => unreachable!(),
    }

    (
        Two(two.unwrap()),
        Five(five.unwrap()),
        Six(six.unwrap()),
        rest,
    )
}

fn deduce_3(patterns: Vec<HashSet<char>>) -> (Three, Vec<HashSet<char>>) {
    let (mut three, rest): (Vec<HashSet<char>>, _) =
        patterns.into_iter().partition(|p| p.len() == 5);
    (Three(three.remove(0)), rest)
}

fn deduce_0_9(four: &Four, mut patterns: Vec<HashSet<char>>) -> (Zero, Nine) {
    if four.0.is_subset(&patterns[0]) {
        (Zero(patterns.remove(1)), Nine(patterns.remove(0)))
    } else {
        (Zero(patterns.remove(0)), Nine(patterns.remove(0)))
    }
}
