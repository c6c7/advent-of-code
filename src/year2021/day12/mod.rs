use std::collections::HashMap;

type Cave<'a> = &'a str;
type CaveMap<'a> = HashMap<Cave<'a>, (bool, Vec<Cave<'a>>)>;
type CavePath<'a> = Vec<Cave<'a>>;

pub fn part1(input: String) {
    let mut cave_map = CaveMap::new();
    input.trim().split("\n").for_each(|l| {
        let mut parts = l.trim().split("-");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();
        insert_edge(&mut cave_map, left, right);
        insert_edge(&mut cave_map, right, left);
    });

    let mut all_paths = vec![];
    let mut root_path = vec![];
    follow(&mut all_paths, &mut cave_map, &mut root_path, "start");

    println!("Number of paths: {}", all_paths.len());
}

fn insert_edge<'a>(cave_map: &mut CaveMap<'a>, from: Cave<'a>, to: Cave<'a>) {
    if let Some((_, neighbors)) = cave_map.get_mut(from) {
        neighbors.push(to);
    } else {
        cave_map.insert(from, (false, vec![to]));
    }
}

fn follow<'a>(
    all_paths: &mut Vec<CavePath<'a>>,
    map: &mut CaveMap<'a>,
    path: &mut Vec<Cave<'a>>,
    to: Cave<'a>,
) {
    path.push(to);
    if to == "end" {
        all_paths.push(path.clone());
    }

    if to.clone().to_uppercase() != to {
        let (visited, _) = map.get_mut(to).unwrap();
        *visited = true;
    }

    let original_length = path.len();
    let neighbors = map.get(to).unwrap().1.clone();
    for c in neighbors {
        if map.get(c).unwrap().0 {
            continue;
        }
        follow(all_paths, map, path, c);
        path.truncate(original_length);
    }

    if to.clone().to_uppercase() != to {
        let (visited, _) = map.get_mut(to).unwrap();
        *visited = false;
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
