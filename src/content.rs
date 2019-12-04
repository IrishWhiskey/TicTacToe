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
}
