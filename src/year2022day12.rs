use petgraph::{algo::dijkstra::dijkstra, graphmap::DiGraphMap, visit::EdgeRef};

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, std::hash::Hash, Debug)]
struct Position(usize, usize);

#[allow(clippy::type_complexity)]
fn build_heightmap(input: &str) -> (DiGraphMap<Position, usize>, DiGraphMap<Position, usize>, Position, Position, Vec<Position>) {
    let mut heightmap = DiGraphMap::new();
    let mut heightmap_reverse = DiGraphMap::new();

    let mut start = None;
    let mut goal = None;
    let mut all_a_positions = vec![];
    let mut heightmap_bytes: Vec<Vec<u8>> = vec![vec![]];

    {
        let mut position = Position(0, 0);
        for byte in input.trim().as_bytes() {
            match *byte {
                b'\n' => {
                    position.1 = 0;
                    position.0 += 1;
                    heightmap_bytes.push(vec![]);
                    continue;
                }
                b'S' => {
                    start.replace(position);
                    all_a_positions.push(position);
                    heightmap_bytes[position.0].push(b'a');
                }
                b'E' => {
                    goal.replace(position);
                    heightmap_bytes[position.0].push(b'z');
                }
                b'a' => {
                    all_a_positions.push(position);
                    heightmap_bytes[position.0].push(b'a');
                }
                byte => {
                    heightmap_bytes[position.0].push(byte);
                }
            }
            heightmap.add_node(position);
            position.1 += 1;
        }
    }

    tracing::debug!("{heightmap_bytes:?}");

    {
        for (i, row) in heightmap_bytes.iter().enumerate() {
            for (j, byte) in row.iter().enumerate() {
                // left
                if j > 0 && heightmap_bytes[i][j - 1] <= byte + 1 {
                    heightmap.add_edge(Position(i, j), Position(i, j - 1), 1);
                    heightmap_reverse.add_edge(Position(i, j - 1), Position(i, j), 1);
                }
                // right
                if j < heightmap_bytes[i].len() - 1 && heightmap_bytes[i][j + 1] <= byte + 1 {
                    heightmap.add_edge(Position(i, j), Position(i, j + 1), 1);
                    heightmap_reverse.add_edge(Position(i, j + 1), Position(i, j), 1);
                }
                // up
                if i > 0 && heightmap_bytes[i - 1][j] <= byte + 1 {
                    heightmap.add_edge(Position(i, j), Position(i - 1, j), 1);
                    heightmap_reverse.add_edge(Position(i - 1, j), Position(i, j), 1);
                }
                // down
                if i < heightmap_bytes.len() - 1 && heightmap_bytes[i + 1][j] <= byte + 1 {
                    heightmap.add_edge(Position(i, j), Position(i + 1, j), 1);
                    heightmap_reverse.add_edge(Position(i + 1, j), Position(i, j), 1);
                }
            }
        }
    }
    (heightmap, heightmap_reverse, start.unwrap(), goal.unwrap(), all_a_positions)
}

pub fn part1(input: &str) {
    let (heightmap, _, start, goal, _) = build_heightmap(input);
    let path_costs = dijkstra(&heightmap, start, Some(goal), |e| *e.weight());
    tracing::debug!("start: {:?}", start);
    tracing::debug!("goal: {:?}", goal);
    let mut x = path_costs.iter().collect::<Vec<_>>();
    x.sort();
    tracing::debug!("{:?}", x);
    tracing::info!(
        "Part 1 Answer: {}",
        path_costs
            .get(&goal)
            .unwrap()
    );
}

pub fn part2(input: &str) {
    let (_, heightmap_reverse, _, goal, all_a_positions) = build_heightmap(input);
    let path_costs = dijkstra(&heightmap_reverse, goal, None, |e| *e.weight());
    tracing::info!("Part 2 Answer: {}", path_costs.iter().filter(|(key, _)| all_a_positions.contains(*key)).min_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1);
}
