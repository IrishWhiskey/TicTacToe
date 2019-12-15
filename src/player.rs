//!module that handles player

#[derive(PartialEq, Copy, Clone)]
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
