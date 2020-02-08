//!module that handles player

use rand;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Player {
    P1,
    P2
}

pub fn get_player_id(player: Player) -> u32 {
    match player {
        Player::P1 => 1,
        Player::P2 => 2,
    }
}

pub fn get_next_player(player: Player) -> Player {
    match player {
        Player::P1 => Player::P2,
        Player::P2 => Player::P1,
    }
}

pub fn get_random_player() -> Player {
    let x = rand::random::<u8>() % 2;
    if x == 0 {
        return Player::P1;
    }
    Player::P2
}

///Handles player ai
pub mod ai {
    use super::Player;
    use crate::content::{Coordinate, Grid};

    ///Returns coordinates of next empty cell
    fn get_move_naive(grid: &Grid) -> Coordinate {
        for i in 0..3 {
            for j in 0..3 {
                let c = Coordinate::new(i, j).unwrap();
                if grid.cell_content(&c).0.is_none() {
                    return c;
                }
            }
        }
        panic!("Something went wrong deciding a move");
    }

    ///Returns ai next move
    pub fn get_move(grid: &Grid, player: Player) -> Coordinate {
        get_move_naive(grid)
    }

    #[cfg(test)]
    mod tests {
        use crate::content::{Grid, Coordinate};
        use super::*;

        #[test]
        pub fn naive_move() {
            let mut g = Grid::new();
            g.player_move(&Coordinate::new(0, 0).unwrap(), Player::P1);
            g.player_move(&Coordinate::new(1, 1).unwrap(), Player::P2);
            let c = get_move_naive(&g);
            assert_eq!(c.row, 0);
            assert_eq!(c.column, 1);
        }
    }
}
