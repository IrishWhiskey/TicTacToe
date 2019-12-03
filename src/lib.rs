pub mod content;
pub mod ui;

pub fn run() {
    let mut grid = content::Grid::new();
    grid.player1_move(1, 2);
    ui::print_grid(&grid);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
