#[derive(Clone, Debug, PartialEq, Eq)]
enum Visibility {
    Visible,
    Hidden,
}

fn determine_visibility(trees: &[u8]) -> Vec<Visibility> {
    let mut trees_visibility = vec![Visibility::Hidden; trees.len()];
    for trees_perspective in [
        trees.iter().enumerate().collect::<Vec<(usize, &u8)>>(),
        trees
            .iter()
            .enumerate()
            .rev()
            .collect::<Vec<(usize, &u8)>>(),
    ] {
        let mut tallest_tree = b'0' - 1;
        for (i, tree) in trees_perspective {
            if *tree > tallest_tree {
                tallest_tree = *tree;
                trees_visibility[i] = Visibility::Visible;
            }
        }
    }
    trees_visibility
}

fn merge_visibility(a: &Visibility, b: &Visibility) -> Visibility {
    if *a == Visibility::Visible || *b == Visibility::Visible {
        return Visibility::Visible;
    }
    Visibility::Hidden
}

fn transpose<T: Clone>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut grid_t = vec![vec![]; n];
    #[allow(clippy::needless_range_loop)]
    for i in 0..m {
        for j in 0..n {
            grid_t[j].push(grid[i][j].clone());
        }
    }
    grid_t
}

fn build_forest(input: &str) -> Vec<Vec<u8>> {
    let mut forest = vec![];
    for line in input.trim().split('\n') {
        forest.push(line.as_bytes().to_vec());
    }
    forest
}

pub fn part1(input: &str) {
    let forest = build_forest(input);
    let mut visibility = vec![];
    for row in &forest {
        visibility.push(determine_visibility(row));
    }

    let forest_t = transpose(&forest);
    let mut visibility_t = transpose(&visibility);
    for (i, col) in forest_t.iter().enumerate() {
        let col_visibility = determine_visibility(col);
        let merged_visibility: Vec<Visibility> = col_visibility
            .iter()
            .zip(visibility_t[i].iter())
            .map(|(a, b)| merge_visibility(a, b))
            .collect();
        visibility_t[i] = merged_visibility;
    }
    let final_visibility = transpose(&visibility_t);

    tracing::debug!("{:?}", forest);
    tracing::debug!("{:?}", final_visibility);
    tracing::info!(
        "Part 1 Answer: {}",
        final_visibility
            .iter()
            .fold(0, |acc, row| row.iter().fold(acc, |acc, v| {
                if *v == Visibility::Visible {
                    acc + 1
                } else {
                    acc
                }
            }))
    );
}

fn determine_scenic_score(trees: &[u8]) -> Vec<usize> {
    let mut scenic_scores = vec![];
    for (i, treehouse) in trees.iter().enumerate() {
        let mut left_score: usize = 0;
        let mut right_score: usize = 0;
        for tree in trees.iter().take(i).rev() {
            left_score += 1;
            if treehouse <= tree {
                break;
            }
        }
        for tree in trees.iter().skip(i + 1) {
            right_score += 1;
            if treehouse <= tree {
                break;
            }
        }
        scenic_scores.push(left_score * right_score)
    }
    scenic_scores
}

pub fn part2(input: &str) {
    let mut forest = build_forest(input);

    let mut scenic_scores = vec![vec![1usize; forest[0].len()]; forest.len()];
    for _ in 0..2 {
        scenic_scores = forest
            .iter()
            .enumerate()
            .fold(scenic_scores, |mut acc, (i, row)| {
                acc[i]
                    .iter_mut()
                    .zip(determine_scenic_score(row).iter())
                    .for_each(|(a, b)| *a *= b);
                acc
            });
        scenic_scores = transpose(&scenic_scores);
        forest = transpose(&forest);
    }
    tracing::debug!("{scenic_scores:?}");
    tracing::info!(
        "Part 2 Answer: {}",
        scenic_scores
            .iter()
            .map(|r| r.iter().max().unwrap())
            .max()
            .unwrap()
    );
}

#[cfg(test)]
mod tests {
    use {super::*, test_case::test_case, tracing_test::traced_test};

    #[traced_test]
    #[test_case("".as_bytes(), vec![]; "empty")]
    #[test_case("0".as_bytes(), vec![Visibility::Visible]; "single_zero")]
    #[test_case("1".as_bytes(), vec![Visibility::Visible]; "single_nonzero")]
    #[test_case("12345678987654321".as_bytes(), vec![Visibility::Visible; 17]; "all_visible")]
    #[test_case("101".as_bytes(), vec![Visibility::Visible, Visibility::Hidden, Visibility::Visible]; "middle_hidden")]
    #[test_case("30373".as_bytes(), vec![Visibility::Visible, Visibility::Hidden, Visibility::Hidden, Visibility::Visible, Visibility::Visible]; "nontrivial")]
    #[test_case("33549".as_bytes(), vec![Visibility::Visible, Visibility::Hidden, Visibility::Visible, Visibility::Hidden, Visibility::Visible]; "missed in example")]
    fn single_row_visibility_test(trees: &[u8], expected_visibility: Vec<Visibility>) {
        assert_eq!(expected_visibility, determine_visibility(trees));
    }

    #[traced_test]
    #[test]
    fn simple_transpose() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![4, 5, 6]],
            transpose(&vec![vec![1, 4], vec![2, 5], vec![3, 6]])
        )
    }

    #[traced_test]
    #[test_case("".as_bytes(), vec![]; "empty")]
    #[test_case("0".as_bytes(), vec![0]; "single zero")]
    #[test_case("9".as_bytes(), vec![0]; "single nonzero")]
    #[test_case("12".as_bytes(), vec![0, 0]; "edges only")]
    #[test_case("25512".as_bytes(), vec![0,1,2,1,0]; "nontrivial")]
    fn single_row_scenic_score(trees: &[u8], expected_scenic_score: Vec<usize>) {
        assert_eq!(expected_scenic_score, determine_scenic_score(trees));
    }
}
