//!module that handles the ui of the game
use std::io::{self, Write};
use crate::content::{Grid, Coordinate};

pub enum MenuChoice {
    SinglePlayer,
    MultiPlayer,
    Quit
}

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

fn read_menu_choice() -> Result<MenuChoice, &'static str> {
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    let choice = read_index()?;
    if choice == 1 {
        return Ok(MenuChoice::SinglePlayer);
    }
    if choice == 2 {
        return Ok(MenuChoice::MultiPlayer);
    }
    if choice == 3 {
        return Ok(MenuChoice::Quit);
    }
    Err("Invalid choice!")
}

fn display_menu() {
    println!("Welcome to TicTacToe!");
    println!("Main Menu");
    println!("1. Singleplayer");
    println!("2. Multiplayer");
    println!("3. Quit");
}

pub fn run_menu() -> MenuChoice {
    loop {
        display_menu();
        match read_menu_choice() {
            Ok(m) => return m,
            Err(e) => println!("Error with menu choice: {}", e),
        }
    }
}

pub fn get_player_move(player_id: u32) -> Result<Coordinate, &'static str> {
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

    Coordinate::new(r as usize, c as usize)
}

pub fn display_winner(winner: u32) {
    println!("The winner is player{}", winner);
}

pub fn display_grid(grid: &Grid)
{
    println!("");
    println!("    0   1   2  ");
    println!("  +---+---+---+");
    for i in 0..3 {
        print!("{} ", i);
        for j in 0..3 {
            print!("| {} ", grid.cell_content(Coordinate::new(i, j).unwrap()));
        }
        println!("|\n  +---+---+---+");
    }
    println!("");
}
