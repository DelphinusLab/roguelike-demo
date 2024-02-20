use self::{combat::Combat, roles::player::Player};
use crate::config::{DEBUG, DEFAULT_HP, DEFAULT_POWER};

pub mod cards;
pub mod combat;
pub(crate) mod roles;

mod state;

#[derive(PartialEq)]
pub enum TurnResult {
    PlayerWin,
    EnemyWin,
    Continue,
}

pub enum Action {
    PlayCard(usize),
    EndTurn,
}

pub struct Engine {
    pub(crate) player: Player,
    pub floor: usize,
}

impl Engine {
    pub fn new_game() -> Self {
        Self {
            player: Player::init(DEFAULT_HP, DEFAULT_POWER, vec![]),
            floor: 0,
        }
    }

    pub fn challenge_next_floor(&mut self) -> Combat {
        let combat = Combat::new(self.floor, &mut self.player);

        self.floor += 1;

        if DEBUG {
            println!("floor {}", self.floor);
        }

        combat
    }
}
