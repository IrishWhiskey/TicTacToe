//!module that handles the ui of the game
use std::io::{self, Write};
use crate::content::*;
use crate::player::{Player, get_player_id};

///Reads index of a row or column
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

pub fn display_message(msg: &str) {
    println!("{}", msg);
}

///Function that handles menu and returns player choice
pub fn run_menu() -> MenuChoice {
    loop {
        display_menu();
        match read_menu_choice() {
            Ok(m) => return m,
            Err(e) => println!("Error with menu choice: {}", e),
        }
    }
}

///Gets the coordinates of player next move
pub fn get_player_move(player: Player) -> Result<Coordinate, &'static str> {
    let r: u32;
    let c: u32;
    let player_id = get_player_id(player);

    print!("Player{} give your move row: ", player_id);
    io::stdout().flush().unwrap();
    r = read_index()?;
    print!("Player{} give your move column: ", player_id);
    io::stdout().flush().unwrap();
    c = read_index()?;

    Coordinate::new(r as usize, c as usize)
}

pub fn announce_move(ai_player: Player, coord: &Coordinate) {
    let player_id = get_player_id(ai_player);
    println!("Player{} moves to ({}, {})", player_id, coord.row, coord.column);
}

pub fn display_winner(player: Player) {
    let player_id = get_player_id(player);
    println!("The winner is Player{}", player_id);
}

fn cell_to_symbol(cell: Cell) -> char {
    match cell.0 {
        Some(Player::P1) => 'X',
        Some(Player::P2) => 'O',
        None => ' ',
    }
}

pub fn display_grid(grid: &Grid)
{
    println!("");
    println!("    0   1   2  ");
    println!("  +---+---+---+");
    for i in 0..3 {
        print!("{} ", i);
        for j in 0..3 {
            let cell = grid.cell_content(&Coordinate::new(i, j).unwrap());
            print!("| {} ", cell_to_symbol(cell));
        }
        println!("|\n  +---+---+---+");
    }
    println!("");
}
