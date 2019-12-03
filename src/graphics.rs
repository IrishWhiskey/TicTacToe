//!module that handles the graphics of the game
use crate::content::Grid;

pub fn print_grid(grid: Grid)
{
    println!("+---+---+---+");
    for i in 0..3 {
        for j in 0..3 {
            print!("| {} ", grid.cell_content(i, j));
        }
        println!("|\n+---+---+---+");
    }
}
