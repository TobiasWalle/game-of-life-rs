#![allow(dead_code)]
use rand::Rng;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Boundary {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

fn wrapping_add(n: usize, max: usize) -> usize {
    match n {
        n if n == max => 0,
        _ => n + 1,
    }
}

fn wrapping_sub(n: usize, max: usize) -> usize {
    match n {
        0 => max,
        _ => n - 1,
    }
}

impl Position {
    fn top(&self, boundary: &Boundary) -> Self {
        Self {
            y: wrapping_sub(self.y, boundary.height - 1),
            ..self.clone()
        }
    }

    fn right(&self, boundary: &Boundary) -> Self {
        Self {
            x: wrapping_add(self.x, boundary.width - 1),
            ..self.clone()
        }
    }

    fn down(&self, boundary: &Boundary) -> Self {
        Self {
            y: wrapping_add(self.y, boundary.height - 1),
            ..self.clone()
        }
    }

    fn left(&self, boundary: &Boundary) -> Self {
        Self {
            x: wrapping_sub(self.x, boundary.width - 1),
            ..self.clone()
        }
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Board {
    boundary: Boundary,
    alive_cells: HashSet<Position>,
}

impl Board {
    pub fn new(boundary: Boundary) -> Self {
        Self {
            boundary,
            ..Self::default()
        }
    }

    pub fn randomize(&mut self) {
        self.alive_cells = HashSet::new();
        let mut rng = rand::thread_rng();
        for y in 0..self.boundary.height {
            for x in 0..self.boundary.width {
                if rng.gen_range(0_f32..1_f32) > 0.8 {
                    self.alive_cells.insert(Position { x, y });
                }
            }
        }
    }

    fn neighbours(&self, p: &Position) -> [Position; 8] {
        let b = &self.boundary;
        [
            p.top(b),
            p.top(b).right(b),
            p.right(b),
            p.down(b).right(b),
            p.down(b),
            p.down(b).left(b),
            p.left(b),
            p.top(b).left(b),
        ]
    }

    fn count_alive_neighbours(&self, p: &Position) -> u8 {
        self.neighbours(p)
            .iter()
            .map(|cell| match self.alive_cells.get(cell) {
                Some(_) => 1,
                None => 0,
            })
            .sum()
    }

    pub fn next_gen(&mut self) {
        let mut all_neighbours = HashSet::new();
        let mut cells_to_kill = HashSet::new();

        for p in &self.alive_cells {
            all_neighbours.extend(self.neighbours(p));
            match self.count_alive_neighbours(p) {
                n if (2..=3).contains(&n) => {
                    // Survive
                }
                _ => {
                    cells_to_kill.insert(p.clone());
                }
            };
        }

        let potential_cells_to_bear = all_neighbours.difference(&self.alive_cells);
        let mut cells_to_bear = HashSet::new();

        for p in potential_cells_to_bear {
            match self.count_alive_neighbours(p) {
                n if n == 3 => {
                    cells_to_bear.insert(p.clone());
                }
                _ => (),
            };
        }

        for p in cells_to_kill {
            self.alive_cells.remove(&p);
        }
        for p in cells_to_bear {
            self.alive_cells.insert(p);
        }
    }

    pub fn pretty(&self) -> String {
        format!("{self}")
    }
}

impl<const W: usize, const H: usize> From<[[u8; W]; H]> for Board {
    fn from(grid: [[u8; W]; H]) -> Self {
        let mut board = Self {
            boundary: Boundary {
                height: H,
                width: W,
            },
            ..Self::default()
        };
        for (y, row) in grid.iter().enumerate() {
            for (x, alive) in row.iter().enumerate() {
                if *alive == 0 {
                    continue;
                }
                let position = Position { x, y };
                board.alive_cells.insert(position);
            }
        }
        board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = vec![vec![false; self.boundary.width]; self.boundary.height];
        for p in &self.alive_cells {
            grid[p.y][p.x] = true;
        }
        for row in grid {
            for is_alive in row {
                if is_alive {
                    write!(f, "👾")?;
                } else {
                    write!(f, "⬛️")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn assert_eq_board(board_1: Board, board_2: Board) {
        assert_eq!(board_1.pretty(), board_2.pretty());
    }

    #[test]
    fn should_die_with_0_neighbours() {
        #[rustfmt::skip]
        let mut board = Board::from([
          [0, 0, 0],
          [0, 1, 0],
          [0, 0, 0],
        ]);

        board.next_gen();

        #[rustfmt::skip]
        let board_expected = Board::from([
          [0, 0, 0],
          [0, 0, 0],
          [0, 0, 0],
        ]);
        assert_eq_board(board, board_expected);
    }

    #[test]
    fn should_die_with_1_neighbours() {
        #[rustfmt::skip]
        let mut board = Board::from([
          [0, 0, 0],
          [0, 1, 1],
          [0, 0, 0],
        ]);

        board.next_gen();

        #[rustfmt::skip]
        let board_expected = Board::from([
          [0, 0, 0],
          [0, 0, 0],
          [0, 0, 0],
        ]);
        assert_eq_board(board, board_expected);
    }

    #[test]
    fn should_not_die_with_2_neighbours_and_be_born_with_3_neighbours() {
        #[rustfmt::skip]
        let mut board = Board::from([
          [0, 0, 0, 0, 0],
          [0, 1, 1, 1, 0],
          [0, 0, 0, 0, 0],
        ]);

        board.next_gen();

        #[rustfmt::skip]
        let board_expected = Board::from([
          [0, 0, 1, 0, 0],
          [0, 0, 1, 0, 0],
          [0, 0, 1, 0, 0],
        ]);
        assert_eq_board(board, board_expected);
    }

    #[test]
    fn should_die_with_4_neighbours() {
        #[rustfmt::skip]
        let mut board = Board::from([
          [0, 0, 0, 0, 0],
          [0, 0, 1, 0, 0],
          [0, 1, 1, 1, 0],
          [0, 0, 1, 0, 0],
          [0, 0, 0, 0, 0],
        ]);

        board.next_gen();

        #[rustfmt::skip]
        let board_expected = Board::from([
          [0, 0, 0, 0, 0],
          [0, 1, 1, 1, 0],
          [0, 1, 0, 1, 0],
          [0, 1, 1, 1, 0],
          [0, 0, 0, 0, 0],
        ]);
        assert_eq_board(board, board_expected);
    }
}
