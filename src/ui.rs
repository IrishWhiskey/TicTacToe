//!module that handles the ui of the game
use crate::content::Grid;
use std::io::{self, Write};

fn read_index() -> Result<u32, &'static str> {
    let mut inp = String::new();
    if let Err(_) = io::stdin().read_line(&mut inp) {
        return Err("Cannot read input!");
    }
    match inp.trim().parse() {
        Ok(ans) => Ok(ans),
        Err(_) => Err("Input in not unsigned integer!"),
    }
}

pub fn get_player_move(player_id: u32) -> Result<(u32, u32), &'static str> {
    if player_id > 1 {
        panic!("Invalid player id!");
    }

    let r: u32;
    let c: u32;

    print!("Player{} give your move row: ", player_id+1);
    io::stdout().flush().unwrap();
    r = read_index()?;
    print!("Player{} give your move column: ", player_id+1);
    io::stdout().flush().unwrap();
    c = read_index()?;

    Ok((r, c))
}

pub fn display_winner(winner: u32) {
    println!("The winner is player{}", winner);
}

pub fn print_grid(grid: &Grid)
{
    println!("");
    println!("    0   1   2  ");
    println!("  +---+---+---+");
    for i in 0..3 {
        print!("{} ", i);
        for j in 0..3 {
            print!("| {} ", grid.cell_content(i, j));
        }
        println!("|\n  +---+---+---+");
    }
    println!("");
}
