use std::convert::TryInto;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

struct OctopusGrid(pub [[u8; GRID_WIDTH]; GRID_HEIGHT]);

impl OctopusGrid {
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, [u8; GRID_WIDTH]> {
        self.0.iter()
    }
}

impl std::ops::Index<usize> for OctopusGrid {
    type Output = [u8; GRID_WIDTH];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for OctopusGrid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..GRID_HEIGHT {
            for j in 0..GRID_WIDTH {
                if self.0[i][j] > 9 {
                    write!(f, "*")?;
                } else {
                    write!(f, "{}", self.0[i][j])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn part1(input: String) {
    let mut octopus_grid = OctopusGrid([[0u8; GRID_WIDTH]; GRID_HEIGHT]);
    input.split("\n").enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            octopus_grid[i][j] = c.to_digit(10).unwrap() as u8;
        });
    });
    println!("Octopus Grid:");
    println!("{}", octopus_grid);

    let mut flashes = 0;
    for i in 0..100000 {
        flashes += advance_step(&mut octopus_grid);
        println!("After step {}:", i + 1);
        println!("Octopus Grid:");
        println!("{}", octopus_grid);

        if simultaneous_flash(&octopus_grid) {
            break;
        }
    }
    println!("Total flashes: {}", flashes);
}

fn simultaneous_flash<'a>(octopus_grid: &'a OctopusGrid) -> bool {
    octopus_grid
        .iter()
        .fold(true, |acc, r| r.iter().fold(acc, |acc, o| acc && *o == 0))
}

fn advance_step(octopus_grid: &mut OctopusGrid) -> u32 {
    let mut flashes = 0;
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            octopus_grid[i][j] += 1;
        }
    }

    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            if octopus_grid[i][j] > 9 {
                flashes += flash(octopus_grid, i, j);
            }
        }
    }
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            if octopus_grid[i][j] > 9 {
                octopus_grid[i][j] = 0;
            }
        }
    }
    flashes
}

fn flash(octopus_grid: &mut OctopusGrid, i: usize, j: usize) -> u32 {
    let mut flashes = 1; // for octopus at (i,j)
    octopus_grid[i][j] = 0;

    for x in -1..2 {
        for y in -1..2 {
            let (a, b) = match ((i as i64 + x).try_into(), (j as i64 + y).try_into()) {
                (Ok(a), Ok(b)) => {
                    if a >= GRID_HEIGHT || b >= GRID_WIDTH {
                        continue;
                    } else {
                        (a, b)
                    }
                }
                _ => continue,
            };

            if octopus_grid[a][b] == 0 {
                continue;
            }

            octopus_grid[a][b] += 1;
            if octopus_grid[a][b] > 9 {
                flashes += flash(octopus_grid, a, b);
            }
        }
    }
    flashes
}
