#![allow(clippy::all)]

fn parse_coordinates(input: &str) -> Vec<(usize, usize)> {
    input
        .split("\n\n")
        .next()
        .unwrap()
        .split("\n")
        .map(|s| {
            let mut parts = s.split(",");
            let x = parts.next().unwrap().parse::<usize>().unwrap();
            let y = parts.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .collect()
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

fn parse_fold_sequence(input: &str) -> Vec<Fold> {
    let mut parts = input.trim().split("\n\n");
    let _ = parts.next();
    parts
        .next()
        .unwrap()
        .split("\n")
        .map(|s| {
            if s.contains("fold along x") {
                let mut parts = s.split("=");
                parts.next().unwrap();
                return Fold::X(parts.next().unwrap().parse::<usize>().unwrap());
            }
            let mut parts = s.split("=");
            parts.next().unwrap();
            Fold::Y(parts.next().unwrap().parse::<usize>().unwrap())
        })
        .collect()
}

pub fn part1(input: String) {
    let (max_x, max_y) =
        parse_coordinates(&input)
            .iter()
            .fold((0, 0), |(mut max_x, mut max_y), (x, y)| {
                if *x > max_x {
                    max_x = *x;
                }
                if *y > max_y {
                    max_y = *y;
                }
                (max_x, max_y)
            });
    let original = {
        let mut original = TransparentPaper(vec![vec![false; max_x + 1]; max_y + 1]);
        parse_coordinates(&input)
            .iter()
            .for_each(|(x, y)| original[*y][*x] = true);
        original
    };
    let fold_sequence = parse_fold_sequence(&input);

    println!("Original shape: {} x {}", original[0].len(), original.len());
    println!("Fold sequence: {:?}", fold_sequence);

    let final_ = fold_sequence.iter().fold(original, |before_fold, fold| {
        let after_fold = match fold {
            Fold::X(x) => fold_along_x(before_fold, *x),
            Fold::Y(y) => fold_along_y(before_fold, *y),
        };
        println!("Total visible: {}", after_fold.total_visible());
        after_fold
    });
    println!("Final:\n{}", final_);
}

#[derive(Debug, PartialEq, Eq)]
struct TransparentPaper(Vec<Vec<bool>>);

impl TransparentPaper {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn total_visible(&self) -> usize {
        self.0.iter().fold(0, |total_acc, row| {
            total_acc
                + row
                    .iter()
                    .fold(0, |row_acc, dot| if *dot { row_acc + 1 } else { row_acc })
        })
    }
}

impl std::ops::Index<usize> for TransparentPaper {
    type Output = Vec<bool>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for TransparentPaper {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::fmt::Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if self.0[i][j] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn fold_along_x(before_fold: TransparentPaper, x: usize) -> TransparentPaper {
    let before_width = before_fold[0].len();
    let after_width = if x > before_width / 2 {
        x - 1
    } else {
        before_fold[0].len() - x - 1
    };

    let n_hanging = if x > before_width / 2 {
        0
    } else {
        (before_width - x) - x - 1
    };

    let mut after_fold = TransparentPaper(vec![vec![false; after_width]; before_fold.len()]);
    for i in 0..before_fold.len() {
        for j in x + 1..before_width - n_hanging {
            let overlap_idx = (x - 1) - (j - (x + 1));
            after_fold[i][n_hanging + overlap_idx] =
                before_fold[i][overlap_idx] || before_fold[i][j];
        }

        if n_hanging > 0 {
            for j in 0..n_hanging {
                after_fold[i][j] = before_fold[i][before_width - 1 - j];
            }
        } else {
            for j in 0..x - (before_width - (x + 1)) {
                after_fold[i][j] = before_fold[i][j];
            }
        }
    }
    after_fold
}

fn fold_along_y(before_fold: TransparentPaper, y: usize) -> TransparentPaper {
    let before_height = before_fold.len();
    let after_height = if y > before_height / 2 {
        y - 1
    } else {
        before_height - y - 1
    };

    let n_hanging = if y > before_height / 2 {
        0
    } else {
        (before_height - y) - y - 1
    };

    let mut after_fold = TransparentPaper(vec![vec![false; before_fold[0].len()]; after_height]);
    for j in 0..before_fold[0].len() {
        for i in y + 1..before_height - n_hanging {
            let overlap_idx = (y - 1) - (i - (y + 1));
            after_fold[n_hanging + overlap_idx][j] =
                before_fold[overlap_idx][j] || before_fold[i][j];
        }

        if n_hanging > 0 {
            for i in 0..n_hanging {
                after_fold[i][j] = before_fold[before_height - 1 - i][j];
            }
        } else {
            for i in 0..y - (before_height - (y + 1)) {
                after_fold[i][j] = before_fold[i][j];
            }
        }
    }
    after_fold
}
