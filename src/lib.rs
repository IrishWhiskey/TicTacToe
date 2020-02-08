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

fn make_ai_move(grid: &mut content::Grid, ai_player: player::Player){
    let ai_move = &player::ai::get_move(grid, ai_player);
    ui::announce_move(ai_player, ai_move);
    &grid.player_move(ai_move, ai_player).unwrap();
}

fn run_multiplayer() {
    let mut grid = content::Grid::new();
    let mut cur_player = player::get_random_player();
    ui::display_message("MultiPlayer Game");

    while grid.winner().is_none() && grid.get_num_moves() < 9 {
        ui::display_grid(&grid);

        if make_user_move(&mut grid, cur_player) {
            cur_player = player::get_next_player(cur_player);
        }
    }

    ui::display_grid(&grid);
    ui::display_winner(grid.winner());
}

fn run_singleplayer() {
    let mut grid = content::Grid::new();
    let mut cur_player = player::get_random_player();
    ui::display_message("SinglePlayer Game\nYou are Player2");

    while grid.winner().is_none()  && grid.get_num_moves() < 9 {
        match cur_player {
            player::Player::P1 => {
                make_ai_move(&mut grid, cur_player)
            }
            player::Player::P2 => {
                ui::display_grid(&grid);
                if !make_user_move(&mut grid, cur_player) {
                    continue;
                }
            }
        }
        cur_player = player::get_next_player(cur_player);
    }

    ui::display_grid(&grid);
    ui::display_winner(grid.winner());
}

///Main function called on execution
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
