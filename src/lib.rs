pub mod content;
pub mod ui;

fn run_multiplayer() {
    let mut grid = content::Grid::new();
    let mut player_id = 0;
    let mut pmove;

    while grid.winner().is_none() {
        ui::display_grid(&grid);

        match ui::get_player_move(player_id) {
            Ok(m) =>  {pmove = m;}
            Err(e) => {
                println!("Error reading player move: {}", e);
                continue;
            }
        }

        if let Err(e) = grid.player_move(pmove, player_id) {
            println!("Error updating grid: {}", e);
            continue;
        }

        player_id = 1-player_id;
    }

    ui::display_grid(&grid);
    ui::display_winner(grid.winner().unwrap()+1);
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
