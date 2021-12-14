use std::collections::{HashMap, HashSet};

type Cave<'a> = &'a str;
type CaveMap<'a> = HashMap<Cave<'a>, Vec<Cave<'a>>>;
type CavePath<'a> = Vec<Cave<'a>>;
type SmallCaveSet<'a> = HashSet<Cave<'a>>;

pub fn part1(input: String) {
    let (mut cave_map, mut small_caves) = parse_input(&input);
    let mut all_paths = HashSet::new();
    let mut root_path = vec![];
    follow(
        &mut all_paths,
        &mut small_caves,
        &mut cave_map,
        &mut root_path,
        "start",
        true,
    );
    println!("Number of paths: {}", all_paths.len());
}

fn parse_input(input: &str) -> (CaveMap, SmallCaveSet) {
    let mut cave_map = CaveMap::new();
    let mut small_caves = SmallCaveSet::new();
    input.trim().split("\n").for_each(|l| {
        let mut parts = l.trim().split("-");
        let left = parts.next().unwrap();
        if left.clone().to_lowercase() == left {
            small_caves.insert(left);
        }
        let right = parts.next().unwrap();
        if right.clone().to_lowercase() == right {
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
    all_paths: &mut HashSet<CavePath<'a>>,
    small_caves: &mut HashSet<Cave<'a>>,
    map: &mut CaveMap<'a>,
    path: &mut Vec<Cave<'a>>,
    to: Cave<'a>,
    double_visit_used: bool,
) {
    let mut next_double_visit_used = double_visit_used;
    path.push(to);
    if to == "end" {
        all_paths.insert(path.clone());
        return;
    }

    if to.clone().to_lowercase() == to {
        if !small_caves.remove(to) {
            if double_visit_used {
                return;
            }
            next_double_visit_used = true;
        }
    }

    let original_length = path.len();
    let neighbors = map.get(to).unwrap().clone();
    for c in neighbors {
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
        cave_map.insert("start", (false, vec!["A", "b"]));
        cave_map.insert("A", (false, vec!["c", "end", "b"]));
        cave_map.insert("c", (false, vec!["A"]));
        cave_map.insert("b", (false, vec!["A", "d", "end"]));
        cave_map.insert("d", (false, vec![]));
        cave_map.insert("end", (false, vec![]));

        let mut all_paths = vec![];
        let mut root_path = vec![];
        follow(&mut all_paths, &mut cave_map, &mut root_path, "start");

        assert_eq!(10, all_paths.len());
    }
}
