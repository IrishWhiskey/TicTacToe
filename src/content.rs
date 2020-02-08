//!module that handles internal data representation

use crate::player::Player;

pub enum MenuChoice {
    SinglePlayer,
    MultiPlayer,
    Quit
}

pub struct Coordinate {
    pub row: usize,
    pub column: usize,
}

///Represents cells of the grid
///It contains the player who occupied the grid (None if cell is empty)
#[derive(PartialEq, Copy, Clone, Debug)]
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
    ///Crates a new grid with empty cells
    pub fn new() -> Grid {
        let c = [[Cell(None); 3]; 3];
        Grid {
            content: c,
        }
    }

    pub fn cell_content(&self, cell: &Coordinate) -> Cell {
        (&self).content[cell.row][cell.column]
    }

    pub fn player_move(&mut self, cell: &Coordinate, player: Player) -> Result<(), &str> {
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

    ///Returns the winner
    pub fn winner(&self) -> Option<Player> {
        self.check()
    }
}

#[cfg(test)]
mod tests {
    use crate::player::Player;
    use super::*;

    #[test]
    #[should_panic]
    fn wrong_cell_content() {
        let g = Grid::new();
        g.cell_content(&Coordinate::new(1, 8).unwrap());
    }

    #[test]
    fn grid_test() {
        let mut g = Grid::new();
        g.player_move(&Coordinate::new(1, 2).unwrap(), Player::P1);
        assert_eq!(Cell(Some(Player::P1)), g.cell_content(&Coordinate::new(1, 2).unwrap()));
    }

    #[test]
    fn no_winner() {
        let mut g = Grid::new();
        g.player_move(&Coordinate::new(1, 2).unwrap(), Player::P1);
        assert!(g.winner().is_none());
    }

    #[test]
    fn winner() {
        let mut g = Grid::new();
        g.player_move(&Coordinate::new(1, 0).unwrap(), Player::P1);
        g.player_move(&Coordinate::new(1, 1).unwrap(), Player::P1);
        g.player_move(&Coordinate::new(1, 2).unwrap(), Player::P1);
        assert_eq!(g.winner().unwrap(), Player::P1);
    }
}
