pub mod ui;
pub mod content;
pub mod player;

fn run_multiplayer() {
    let mut grid = content::Grid::new();
    let mut cur_player = player::Player::P1;
    let mut pmove;

    while grid.winner().is_none() {
        ui::display_grid(&grid);

        match ui::get_player_move(cur_player) {
            Ok(m) =>  {pmove = m;}
            Err(e) => {
                println!("Error reading player move: {}", e);
                continue;
            }
        }

        if let Err(e) = grid.player_move(pmove, cur_player) {
            println!("Error updating grid: {}", e);
            continue;
        }

        cur_player = player::get_next_player(cur_player);
    }

    ui::display_grid(&grid);
    ui::display_winner(grid.winner().unwrap());
}

fn run_singleplayer() {
    println!("SinglePlayer game...");
}

pub fn run()
{
    loop {
        match ui::run_menu() {
            content::MenuChoice::SinglePlayer => run_singleplayer(),
            content::MenuChoice::MultiPlayer => run_multiplayer(),
            content::MenuChoice::Quit => break,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
