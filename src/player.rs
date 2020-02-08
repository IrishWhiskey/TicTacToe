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
    use super::*;
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

    ///If player can win in one move, the function returns that move
    fn get_last_move(mut grid: Grid, player: Player) -> Option<Coordinate> {
        for i in 0..3 {
            for j in 0..3 {
                let c = Coordinate::new(i, j).unwrap();
                if grid.cell_content(&c).0.is_none() {
                    grid.player_move(&c, player).unwrap();
                    if grid.winner() == Some(player) {
                        return Some(c);
                    }
                    grid.clear_cell(&c);
                }
            }
        }
        None
    }

    ///If there is an occupied corner, the function returns the opposite one(if empty)
    fn get_opposite_corner(grid: &Grid) -> Option<Coordinate> {
        for i in 0..2 {
            for j in 0..2 {
                let c = Coordinate::new(2*i, 2*j).unwrap();
                if grid.cell_content(&c).0.is_some() {
                    let c = Coordinate::new((2*i+2)%4, (2*j+2)%4).unwrap();
                    if grid.cell_content(&c).0.is_none() {
                        return Some(c);
                    }
                }
            }
        }

        None
    }

    ///Searches for an empty corner
    fn get_empty_corner(grid: &Grid) -> Option<Coordinate> {
        for i in 0..2 {
            for j in 0..2 {
                let c = Coordinate::new(2*i, 2*j).unwrap();
                if grid.cell_content(&c).0.is_none() {
                    return Some(c);
                }
            }
        }

        None
    }

    ///Tries to return best move
    fn get_smart_move(grid: &Grid, player: Player) -> Coordinate {

        //Try to win in one move
        if let Some(coord) = get_last_move(grid.clone(), player) {
            return coord;
        }

        //Block enemy from winning
        if let Some(coord) = get_last_move(grid.clone(), get_next_player(player)) {
            return coord;
        }

        //Always to move in the middle cell
        let coord = Coordinate::new(1, 1).unwrap();
        if grid.cell_content(&coord).0.is_none() {
            return coord;
        }

        if grid.get_num_moves() == 3 {
            //If player is getting into a trap move in a side cell
            for i in 0..2 {
                for j in 0..2 {
                    let c = Coordinate::new(2*i, 2*j).unwrap();
                    let cc = Coordinate::new((2*i+2)%4, (2*j+2)%4).unwrap();
                    let enemy = get_next_player(player);
                    if grid.occupied_by_player(&c, enemy) && grid.occupied_by_player(&cc, enemy) {
                        return Coordinate::new(0, 1).unwrap();
                    }
                }
            }
        }

        if let Some(coord) = get_opposite_corner(grid) {
            return coord;
        }

        if let Some(coord) = get_empty_corner(grid) {
            return coord;
        }

        get_move_naive(grid)
    }

    ///Returns ai next move
    pub fn get_move(grid: &Grid, player: Player) -> Coordinate {
        get_smart_move(grid, player)
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
