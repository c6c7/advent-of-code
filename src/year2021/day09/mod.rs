#![allow(clippy::all)]

use std::collections::BinaryHeap;

const INPUT_WIDTH: usize = 100;
const INPUT_HEIGHT: usize = 100;

pub fn part1(input: String) {
    let heightmap = Heightmap::new(INPUT_WIDTH, INPUT_HEIGHT, &input);
    let low_point_risk_level_sum = (0..INPUT_HEIGHT).fold(0, |total_acc, i| {
        total_acc + (0..INPUT_WIDTH).fold(0, |row_acc, j| row_acc + heightmap.risk_level(i, j))
    });
    println!("Answer: {}", low_point_risk_level_sum);
}

pub fn part2(input: String) {
    let mut heightmap = Heightmap::new(INPUT_WIDTH, INPUT_HEIGHT, &input);
    let mut basins = heightmap.basin_label();
    let mut answer = 1;
    println!("Basin 1: {:?}", {
        let b = basins.pop().unwrap();
        answer *= b.size;
        b
    });
    println!("Basin 2: {:?}", {
        let b = basins.pop().unwrap();
        answer *= b.size;
        b
    });
    println!("Basin 3: {:?}", {
        let b = basins.pop().unwrap();
        answer *= b.size;
        b
    });
    println!("Answer: {}", answer);
}

#[derive(Clone)]
struct Spot {
    pub height: u32,
    pub visited: bool,
    pub basin_index: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Basin {
    pub size: usize,
    pub seed: (usize, usize),
}

struct Heightmap {
    map: Vec<Vec<Spot>>,
    basins: Vec<Basin>,
}

impl Heightmap {
    pub fn new(width: usize, height: usize, input: &str) -> Heightmap {
        let mut hm = Heightmap {
            map: vec![
                vec![
                    Spot {
                        height: 9,
                        visited: false,
                        basin_index: -1,
                    };
                    width + 2
                ];
                height + 2
            ],
            basins: vec![],
        };
        input.split("\n").enumerate().for_each(|(i, l)| {
            l.trim().chars().enumerate().for_each(|(j, c)| {
                hm.map[i + 1][j + 1].height = c.to_digit(10).unwrap();
            })
        });
        hm
    }

    fn risk_level(&self, x: usize, y: usize) -> u32 {
        let x = x + 1;
        let y = y + 1;

        let h = self.map[x][y].height;
        if self.map[x - 1][y].height > h
            && self.map[x + 1][y].height > h
            && self.map[x][y - 1].height > h
            && self.map[x][y + 1].height > h
        {
            1 + h
        } else {
            0
        }
    }

    fn basin_label<'a>(&'a mut self) -> BinaryHeap<&'a Basin> {
        for i in 0..self.map.len() - 2 {
            for j in 0..self.map[i].len() - 2 {
                let i = i + 1;
                let j = j + 1;

                if self.map[i][j].height == 9 || self.map[i][j].visited {
                    continue;
                }
                let mut basin = Basin {
                    seed: (i - 1, j - 1),
                    size: 0,
                };
                let basin_index = self.basins.len() as i64;

                self.map[i][j].visited = true;
                let mut queue = vec![(i, j)];
                while !queue.is_empty() {
                    let (x, y) = queue.pop().unwrap();
                    self.map[x][y].basin_index = basin_index;
                    basin.size += 1;

                    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                        .into_iter()
                        .for_each(|(a, b)| {
                            if !(self.map[a][b].visited || self.map[a][b].height == 9) {
                                self.map[a][b].visited = true;
                                queue.push((a, b));
                            }
                        });
                }
                self.basins.push(basin);
            }
        }
        self.basins.iter().fold(BinaryHeap::new(), |mut heap, b| {
            heap.push(b);
            heap
        })
    }
}
