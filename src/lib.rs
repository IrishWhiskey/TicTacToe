pub mod content;
pub mod ui;

pub fn run() {
    let mut grid = content::Grid::new();
    let mut player_id = 0;
    let mut pmove: (u32, u32);

    while grid.winner().is_none() {
        ui::print_grid(&grid);

        match ui::get_player_move(player_id) {
            Ok(m) =>  {pmove = m;}
            Err(e) => {
                println!("Error reading player move: {}", e);
                continue;
            }
        }

        if let Err(e) = grid.player_move(pmove.0 as usize, pmove.1 as usize, player_id) {
            println!("Error updating grid: {}", e);
            continue;
        }

        player_id = 1-player_id;
    }

    ui::print_grid(&grid);
    ui::display_winner(grid.winner().unwrap()+1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
