//!module that handles internal data representation

use crate::player::Player;

pub enum MenuChoice {
    SinglePlayer,
    MultiPlayer,
    Quit
}

pub struct Coordinate {
    row: usize,
    column: usize,
}

#[derive(PartialEq, Copy, Clone)]
pub struct Cell(pub Option<Player>);

pub struct Grid {
    content: [[Cell; 3]; 3],
}

impl Coordinate {
    pub fn new(r: usize, c: usize) -> Result<Coordinate, &'static str> {
        if (r > 2) || (c > 2) {
            return Err("row or column index out of bound");
        }
        Ok(Coordinate {
            row: r,
            column: c,
        })
    }
}

impl Grid {
    pub fn new() -> Grid {
        let c = [[Cell(None); 3]; 3];
        Grid {
            content: c,
        }
    }

    pub fn cell_content(&self, cell: Coordinate) -> Cell {
        (&self).content[cell.row][cell.column]
    }

    pub fn player_move(&mut self, cell: Coordinate, player: Player) -> Result<(), &str> {
        if self.content[cell.row][cell.column].0.is_some() {
            return Err("non empty cell");
        }

        self.content[cell.row][cell.column].0 = Some(player);
        Ok(())
    }

    fn check_row(&self, r: usize) -> bool {
        if r > 2 {
            panic!("Row index out of bound!");
        }

        if self.content[r][0] != Cell(None) {
            if self.content[r][0] == self.content[r][1] &&
                self.content[r][1] == self.content[r][2] {
                    return true;
            }
        }

        false
    }

    fn check_col(&self, c: usize) -> bool {
        if c > 2 {
            panic!("Column index out of bound!");
        }

        if self.content[0][c] != Cell(None) {
            if self.content[0][c] == self.content[1][c] &&
                self.content[1][c] == self.content[2][c] {
                    return true;
            }
        }

        false
    }

    fn check_diagonals(&self) -> bool {
        if self.content[1][1] != Cell(None) {
            if self.content[0][0] == self.content[1][1] &&
                self.content[1][1] == self.content[2][2] {
                    return true;
            }

            if self.content[0][2] == self.content[1][1] &&
                self.content[1][1] == self.content[2][0] {
                    return true;
            }
        }

        false
    }

    fn check(&self) -> Option<Player> {
        for i in 0..3 {
            if self.check_col(i) {
                return self.content[0][i].0;
            }

            if self.check_row(i) {
                return self.content[i][0].0;
            }
        }

        if self.check_diagonals() {
            return self.content[1][1].0;
        }

        None
    }

    pub fn winner(&self) -> Option<Player> {
        self.check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn wrong_player_move() {
        let mut g = Grid::new();
        g.player_move(1, 2, 5);
    }

    #[test]
    #[should_panic]
    fn wrong_cell_content() {
        let mut g = Grid::new();
        g.cell_content(1, 8);
    }

    #[test]
    fn grid_test() {
        let mut g = Grid::new();
        g.player_move(1, 2, 0);
        assert_eq!('X', g.cell_content(1, 2));
    }

    #[test]
    fn no_winner() {
        let mut g = Grid::new();
        g.player_move(1, 2, 0);
        assert!(g.winner().is_none());
    }

    #[test]
    fn winner() {
        let mut g = Grid::new();
        g.player_move(1, 0, 0);
        g.player_move(1, 1, 0);
        g.player_move(1, 2, 0);
        assert_eq!(g.winner().unwrap(), 0);
    }
}
