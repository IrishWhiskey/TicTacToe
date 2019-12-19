pub mod ui;
pub mod content;
pub mod player;

fn make_user_move(grid: &mut content::Grid, user: player::Player) -> bool {
    let pmove;

    match ui::get_player_move(user) {
        Ok(m) =>  {pmove = m;}
        Err(e) => {
            println!("Error reading player move: {}", e);
            return false;
        }
    }

    if let Err(e) = &grid.player_move(&pmove, user) {
        println!("Error updating grid: {}", e);
        return false;
    }

    true
}

fn run_multiplayer() {
    let mut grid = content::Grid::new();
    let mut cur_player = player::get_random_player();

    while grid.winner().is_none() {
        ui::display_grid(&grid);

        if make_user_move(&mut grid, cur_player) {
            cur_player = player::get_next_player(cur_player);
        }
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
