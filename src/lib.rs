pub mod content;
pub mod graphics;

pub fn run() {
    let mut grid = content::Grid::new();
    grid.player1_move(0, 0);
    grid.player2_move(1, 2);
    graphics::print_grid(grid);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
