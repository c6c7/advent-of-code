#![allow(clippy::all)]

fn parse_line(line: &str) -> ((usize, usize), (usize, usize)) {
    let mut parts = line.trim().split(" -> ");
    let mut begin_part = parts.next().unwrap().split(",");
    let mut end_part = parts.next().unwrap().split(",");

    let x1 = begin_part.next().unwrap().parse::<usize>().unwrap();
    let y1 = begin_part.next().unwrap().parse::<usize>().unwrap();
    let x2 = end_part.next().unwrap().parse::<usize>().unwrap();
    let y2 = end_part.next().unwrap().parse::<usize>().unwrap();

    ((x1, y1), (x2, y2))
}

pub fn part1(input: String) {
    let lines = input.split("\n");
    let hydrothermal_vents = lines.map(|l| parse_line(l)).collect::<Vec<_>>();
    let (max_x, max_y) =
        hydrothermal_vents
            .iter()
            .fold((0, 0), |(mut max_x, mut max_y), (begin, end)| {
                if begin.0 > max_x {
                    max_x = begin.0;
                }
                if end.0 > max_x {
                    max_x = begin.0;
                }
                if begin.1 > max_y {
                    max_y = begin.1;
                }
                if end.1 > max_y {
                    max_y = begin.1;
                }
                (max_x, max_y)
            });
    println!("{:?}", hydrothermal_vents[0]);
    println!("Max X: {}, Max Y: {}", max_x, max_y);

    let mut grid = [[0usize; 1000]; 1000];

    hydrothermal_vents.iter().for_each(|(begin, end)| {
        if begin.0 != end.0 && begin.1 != end.1 {
            let begin = (begin.0 as i64, begin.1 as i64);
            let end = (end.0 as i64, end.1 as i64);

            // (end.1 - begin.1)/(end.0 - begin.0)*(x - begin.0) = y - begin.1
            let (x_start, x_end) = match begin.0.cmp(&end.0) {
                std::cmp::Ordering::Greater => (end.0, begin.0),
                std::cmp::Ordering::Less => (begin.0, end.0),
                _ => unreachable!(),
            };
            for x in x_start..x_end + 1 {
                let y = ((end.1 - begin.1) * (x - begin.0)) / (end.0 - begin.0) + begin.1;
                grid[x as usize][y as usize] += 1;
            }
            return;
        }

        if begin.0 == end.0 {
            let (start, end) = match begin.1.cmp(&end.1) {
                std::cmp::Ordering::Equal => (begin.1, end.1),
                std::cmp::Ordering::Greater => (end.1, begin.1),
                std::cmp::Ordering::Less => (begin.1, end.1),
            };
            for j in start..end + 1 {
                grid[begin.0][j] += 1;
            }
            return;
        }

        if begin.1 == end.1 {
            let (start, end) = match begin.0.cmp(&end.0) {
                std::cmp::Ordering::Equal => (begin.0, end.0),
                std::cmp::Ordering::Greater => (end.0, begin.0),
                std::cmp::Ordering::Less => (begin.0, end.0),
            };
            for i in start..end + 1 {
                grid[i][begin.1] += 1;
            }
            return;
        }
        unreachable!();
    });

    let mut overlaps = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i][j] >= 2 {
                overlaps += 1;
            }
        }
    }
    println!("Overlaps: {}", overlaps);
}
