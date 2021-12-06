fn parse_number_order(line: &str) -> Vec<u8> {
    line.split(",")
        .map(|n| n.parse::<u8>().expect("Unable to parse number into u8."))
        .collect()
}

#[derive(Debug, Copy, Clone)]
enum BoardSpot {
    Marked(u8),
    Unmarked(u8),
}

#[derive(Debug)]
struct Board([[BoardSpot; 5]; 5]);

impl Board {
    fn new(board_str: &str) -> Board {
        let mut board = Board([[BoardSpot::Unmarked(0); 5]; 5]);
        board_str
            .trim()
            .split("\n")
            .enumerate()
            .for_each(|(i, board_line)| {
                board_line
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse::<u8>().expect("Unable to parse number on board."))
                    .enumerate()
                    .for_each(|(j, n)| board.0[i][j] = BoardSpot::Unmarked(n));
            });
        board
    }

    fn horizontal_bingo(&self) -> bool {
        for i in 0..5 {
            let mut bingo = true;
            for j in 0..5 {
                match self.0[i][j] {
                    BoardSpot::Unmarked(_) => {
                        bingo = false;
                        break;
                    }
                    BoardSpot::Marked(_) => continue,
                }
            }
            if bingo {
                return true;
            }
        }
        false
    }

    fn vertical_bingo(&self) -> bool {
        for j in 0..5 {
            let mut bingo = true;
            for i in 0..5 {
                match self.0[i][j] {
                    BoardSpot::Unmarked(_) => {
                        bingo = false;
                        break;
                    }
                    BoardSpot::Marked(_) => continue,
                }
            }
            if bingo {
                return true;
            }
        }
        false
    }

    fn down_diagonal_bingo(&self) -> bool {
        let mut bingo = true;
        for i in 0..5 {
            match self.0[i][i] {
                BoardSpot::Unmarked(_) => {
                    bingo = false;
                    break;
                }
                BoardSpot::Marked(_) => continue,
            }
        }
        bingo
    }

    fn up_diagonal_bingo(&self) -> bool {
        let mut bingo = true;
        for i in 0..5 {
            match self.0[5 - 1 - i][i] {
                BoardSpot::Unmarked(_) => {
                    bingo = false;
                    break;
                }
                BoardSpot::Marked(_) => continue,
            }
        }
        bingo
    }

    fn mark(&mut self, number: u8) {
        for i in 0..5 {
            for j in 0..5 {
                match self.0[i][j] {
                    BoardSpot::Unmarked(n) if n == number => {
                        self.0[i][j] = BoardSpot::Marked(number)
                    }
                    BoardSpot::Marked(n) if n == number => {
                        panic!("Missed marking a number earlier: {}", number)
                    }
                    _ => {}
                }
            }
        }
    }

    fn bingo(&self) -> bool {
        self.horizontal_bingo()
            || self.vertical_bingo()
            || self.down_diagonal_bingo()
            || self.up_diagonal_bingo()
    }

    fn sum_unmarked(&self) -> u64 {
        let mut sum: u64 = 0;
        for i in 0..5 {
            for j in 0..5 {
                match self.0[i][j] {
                    BoardSpot::Unmarked(n) => sum += n as u64,
                    BoardSpot::Marked(_) => {}
                }
            }
        }
        sum
    }
}

pub fn part1(input: String) {
    let mut parts = input.split("\n\n");
    let number_order = parse_number_order(parts.next().unwrap());
    println!("Number Order: {:?}", number_order);
    let mut all_boards = parts
        .enumerate()
        .fold(vec![], |mut all_boards, (i, board_str)| {
            all_boards.push((i, Board::new(board_str)));
            all_boards
        });

    let _ = number_order.into_iter().try_for_each(|number| {
        all_boards.iter_mut().try_for_each(|(i, board)| {
            board.mark(number);
            if board.bingo() {
                println!("Bingo on: {}", i);
                println!("Puzzle Answer: {}", board.sum_unmarked() * (number as u64));
                return Err(());
            }
            return Ok(());
        })
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_horizontal() {
        let mut board = Board::new(
            r#"
1 1 1 1 1
2 3 4 5 6
2 3 4 5 6
2 3 4 5 6
2 3 4 5 6
"#,
        );
        assert!(!board.horizontal_bingo());
        board.mark(1);
        assert!(board.horizontal_bingo());
    }

    #[test]
    fn other_horizontal() {
        let mut board = Board::new(
            r#"
1 1 1 1 1
2 3 4 5 6
2 2 2 2 2
2 3 4 5 6
2 3 4 5 6
"#,
        );
        assert!(!board.horizontal_bingo());
        board.mark(2);
        assert!(board.horizontal_bingo());
    }

    #[test]
    fn different_numbers_horizontal() {
        let mut board = Board::new(
            r#"
1 1 1 1 1
2 3 4 5 6
2 2 2 2 2
9 3 4 5 6
2 3 4 5 6
"#,
        );
        assert!(!board.horizontal_bingo());
        board.mark(9);
        assert!(!board.horizontal_bingo());
        board.mark(3);
        assert!(!board.horizontal_bingo());
        board.mark(4);
        assert!(!board.horizontal_bingo());
        board.mark(5);
        assert!(!board.horizontal_bingo());
        board.mark(6);
        assert!(board.horizontal_bingo());
    }

    #[test]
    fn first_vertical() {
        let mut board = Board::new(
            r#"
2 4 5 6 7
2 3 4 5 6
2 3 4 5 6
2 3 4 5 6
2 3 4 5 6
"#,
        );
        assert!(!board.vertical_bingo());
        board.mark(2);
        assert!(board.vertical_bingo());
    }

    #[test]
    fn other_vertical() {
        let mut board = Board::new(
            r#"
3 4 4 6 7
2 3 4 5 6
2 3 4 5 6
2 3 4 5 6
2 3 4 5 6
"#,
        );
        assert!(!board.vertical_bingo());
        board.mark(4);
        assert!(board.vertical_bingo());
    }

    #[test]
    fn different_numbers_vertical() {
        let mut board = Board::new(
            r#"
1 1 1 1 7
1 1 1 1 6
1 1 1 1 5
1 1 1 1 4
1 1 1 1 3
"#,
        );
        assert!(!board.vertical_bingo());
        board.mark(7);
        assert!(!board.vertical_bingo());
        board.mark(6);
        assert!(!board.vertical_bingo());
        board.mark(5);
        assert!(!board.vertical_bingo());
        board.mark(4);
        assert!(!board.vertical_bingo());
        board.mark(3);
        assert!(board.vertical_bingo());
    }

    #[test]
    fn simple_down_diagonal() {
        let mut board = Board::new(
            r#"
1 2 2 2 2
2 1 2 2 2
2 2 1 2 2
2 2 2 1 2
2 2 2 2 1
"#,
        );
        assert!(!board.down_diagonal_bingo());
        board.mark(1);
        assert!(board.down_diagonal_bingo());
    }

    #[test]
    fn different_number_down_diagonal() {
        let mut board = Board::new(
            r#"
1 2 2 2 2
2 3 2 2 2
2 2 4 2 2
2 2 2 5 2
2 2 2 2 6
"#,
        );
        assert!(!board.down_diagonal_bingo());
        board.mark(1);
        assert!(!board.down_diagonal_bingo());
        board.mark(3);
        assert!(!board.down_diagonal_bingo());
        board.mark(4);
        assert!(!board.down_diagonal_bingo());
        board.mark(5);
        assert!(!board.down_diagonal_bingo());
        board.mark(6);
        assert!(board.down_diagonal_bingo());
    }

    #[test]
    fn simple_up_diagonal() {
        let mut board = Board::new(
            r#"
2 2 2 2 6
2 2 2 5 2
2 2 4 2 2
2 3 2 2 2
1 2 2 2 2
"#,
        );
        assert!(!board.up_diagonal_bingo());
        board.mark(1);
        assert!(!board.up_diagonal_bingo());
        board.mark(3);
        assert!(!board.up_diagonal_bingo());
        board.mark(4);
        assert!(!board.up_diagonal_bingo());
        board.mark(5);
        assert!(!board.up_diagonal_bingo());
        board.mark(6);
        assert!(board.up_diagonal_bingo());
    }
}
