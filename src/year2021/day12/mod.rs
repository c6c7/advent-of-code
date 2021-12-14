use std::collections::{HashMap, HashSet};

type Cave<'a> = &'a str;
type CaveMap<'a> = HashMap<Cave<'a>, Vec<Cave<'a>>>;
type SmallCaveSet<'a> = HashSet<Cave<'a>>;

pub fn part1(input: String) {
    let (mut cave_map, mut small_caves) = parse_input(&input);
    let mut all_paths = 0;
    let mut root_path = vec![];
    follow(
        &mut all_paths,
        &mut small_caves,
        &mut cave_map,
        &mut root_path,
        "start",
        true,
    );
    println!("Number of paths: {}", all_paths);
}

pub fn part2(input: String) {
    let (mut cave_map, mut small_caves) = parse_input(&input);
    let mut all_paths = 0;
    let mut root_path = vec![];
    follow(
        &mut all_paths,
        &mut small_caves,
        &mut cave_map,
        &mut root_path,
        "start",
        false,
    );
    println!("Number of paths: {}", all_paths);
}

fn parse_input(input: &str) -> (CaveMap, SmallCaveSet) {
    let mut cave_map = CaveMap::new();
    let mut small_caves = SmallCaveSet::new();
    input.trim().split("\n").for_each(|l| {
        let mut parts = l.trim().split("-");
        let left = parts.next().unwrap();
        if left.chars().next().unwrap().is_lowercase() {
            small_caves.insert(left);
        }
        let right = parts.next().unwrap();
        if right.chars().next().unwrap().is_lowercase() {
            small_caves.insert(right);
        }
        if left != "end" && right != "start" {
            insert_edge(&mut cave_map, left, right);
        }
        if right != "end" && left != "start" {
            insert_edge(&mut cave_map, right, left);
        }
    });
    (cave_map, small_caves)
}

fn insert_edge<'a>(cave_map: &mut CaveMap<'a>, from: Cave<'a>, to: Cave<'a>) {
    if let Some(neighbors) = cave_map.get_mut(from) {
        neighbors.push(to);
    } else {
        cave_map.insert(from, vec![to]);
    }
}

fn follow<'a>(
    all_paths: &mut usize,
    small_caves: &mut HashSet<Cave<'a>>,
    map: &CaveMap<'a>,
    path: &mut Vec<Cave<'a>>,
    to: Cave<'a>,
    double_visit_used: bool,
) {
    let mut next_double_visit_used = double_visit_used;
    path.push(to);
    if to == "end" {
        *all_paths += 1;
        return;
    }

    if to.chars().next().unwrap().is_lowercase() {
        if !small_caves.remove(to) {
            if double_visit_used {
                return;
            }
            next_double_visit_used = true;
        }
    }

    let original_length = path.len();
    for c in map.get(to).unwrap() {
        follow(all_paths, small_caves, map, path, c, next_double_visit_used);
        path.truncate(original_length);
    }

    if to.clone().to_lowercase() == to && next_double_visit_used == double_visit_used {
        small_caves.insert(to);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_mocked_cave_map() {
        let mut cave_map = CaveMap::new();
        cave_map.insert("start", vec!["A", "b"]);
        cave_map.insert("A", vec!["c", "end", "b"]);
        cave_map.insert("c", vec!["A"]);
        cave_map.insert("b", vec!["A", "d", "end"]);
        cave_map.insert("d", vec![]);
        cave_map.insert("end", vec![]);

        let mut small_caves = SmallCaveSet::new();
        small_caves.insert("c");
        small_caves.insert("b");
        small_caves.insert("d");

        let mut all_paths = 0;
        let mut root_path = vec![];
        follow(
            &mut all_paths,
            &mut small_caves,
            &mut cave_map,
            &mut root_path,
            "start",
            false,
        );

        assert_eq!(10, all_paths);
    }
}
