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

    fn update_cell(&mut self, r: usize, c: usize, p: CellContent) -> Result<(), &str> {
        if (r > 2) || (c > 2) {
            return Err("row or column index out of bound");
        }
        if self.content[r][c] != CellContent::Nil {
            return Err("non empty cell");
        }
        self.content[r][c] = p;
        Ok(())
    }

    pub fn player1_move(&mut self, r: usize, c: usize) -> Result<(), &str> {
        self.update_cell(r, c, CellContent::Ph1)?;
        Ok(())
    }

    pub fn player2_move(&mut self, r: usize, c: usize) -> Result<(), &str> {
        self.update_cell(r, c, CellContent::Ph2)?;
        Ok(())
    }
}
