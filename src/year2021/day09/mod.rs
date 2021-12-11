const INPUT_WIDTH: usize = 100;
const INPUT_HEIGHT: usize = 100;

pub fn part1(input: String) {
    let heightmap = Heightmap::new(INPUT_WIDTH, INPUT_HEIGHT, &input);
    let low_point_risk_level_sum = (0..INPUT_HEIGHT).fold(0, |total_acc, i| {
        total_acc + (0..INPUT_WIDTH).fold(0, |row_acc, j| row_acc + heightmap.risk_level(i, j))
    });
    println!("Answer: {}", low_point_risk_level_sum);
}

struct Heightmap(Vec<Vec<u32>>);

impl Heightmap {
    pub fn new(width: usize, height: usize, input: &str) -> Heightmap {
        let mut hm = Heightmap(vec![vec![9; width + 2]; height + 2]);
        input.split("\n").enumerate().for_each(|(i, l)| {
            l.trim().chars().enumerate().for_each(|(j, c)| {
                hm.0[i + 1][j + 1] = c.to_digit(10).unwrap();
            })
        });
        hm
    }

    fn risk_level(&self, x: usize, y: usize) -> u32 {
        let x = x + 1;
        let y = y + 1;

        let h = self.0[x][y];
        if self.0[x - 1][y] > h
            && self.0[x + 1][y] > h
            && self.0[x][y - 1] > h
            && self.0[x][y + 1] > h
        {
            1 + h
        } else {
            0
        }
    }
}
