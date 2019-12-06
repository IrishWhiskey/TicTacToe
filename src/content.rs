//!module that handles internal data representation

///A cell contains either a placeholder or nothing(Nil)
#[derive(PartialEq)]
enum CellContent {
    Ph1,
    Ph2,
    Nil,
}

pub struct Grid {
    content: [[CellContent; 3]; 3],
}

impl Grid {
    pub fn new() -> Grid {
        let c = [
        [CellContent::Nil, CellContent::Nil, CellContent::Nil],
        [CellContent::Nil, CellContent::Nil, CellContent::Nil],
        [CellContent::Nil, CellContent::Nil, CellContent::Nil],
        ];
        Grid {
            content: c,
        }
    }

    pub fn cell_content(&self, r: usize, c: usize) -> char {
        if (r > 2) || (c > 2) {
            panic!("Cannot get cell content, row or column index out of bound");
        }

        match &self.content[r][c] {
            CellContent::Ph1 => 'X',
            CellContent::Ph2 => 'O',
            CellContent::Nil => ' ',
        }
    }

    fn update_cell(&mut self, r: usize, c: usize, ph: CellContent) -> Result<(), &str> {
        if (r > 2) || (c > 2) {
            return Err("row or column index out of bound");
        }
        if self.content[r][c] != CellContent::Nil {
            return Err("non empty cell");
        }
        self.content[r][c] = ph;
        Ok(())
    }

    pub fn player_move(&mut self, r: usize, c: usize, player_id: u32) -> Result<(), &str> {
        if player_id > 1 {
            panic!("Invalid player id!");
        }

        let place_holder = if player_id == 0 {
            CellContent::Ph1
        } else {
            CellContent::Ph2
        };

        self.update_cell(r, c, place_holder)?;
        Ok(())
    }

    fn check_row(&self, r: usize) -> bool {
        if r > 2 {
            panic!("Row index out of bound!");
        }

        if self.content[r][0] != CellContent::Nil {
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

        if self.content[0][c] != CellContent::Nil {
            if self.content[0][c] == self.content[1][c] &&
                self.content[1][c] == self.content[2][c] {
                    return true;
            }
        }

        false
    }

    fn check_diagonals(&self) -> bool {
        if self.content[1][1] != CellContent::Nil {
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

    fn check(&self) -> Option<&CellContent> {
        for i in 0..2 {
            if self.check_col(i) {
                return Some(&self.content[0][i]);
            }

            if self.check_row(i) {
                return Some(&self.content[i][0]);
            }
        }

        if self.check_diagonals() {
            return Some(&self.content[1][1]);
        }

        None
    }

    pub fn winner(&self) -> Option<u32> {
        match self.check() {
            Some(CellContent::Ph1) => Some(0),
            Some(CellContent::Ph2) => Some(1),
            _ => None,
        }
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
