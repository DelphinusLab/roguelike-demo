use rand::Rng;

use self::{cards::SkillEffect, combat::Combat, roles::player::Player};
use crate::config::{DEBUG, DEFAULT_HP, DEFAULT_POWER};

pub mod combat;

mod cards;
mod roles;
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

pub struct Engine<R: Rng> {
    pub(crate) player: Player<R>,
    pub floor: usize,
}

impl<R: Rng> Engine<R> {
    pub fn new_game() -> Self {
        Self {
            player: Player::init(DEFAULT_HP, DEFAULT_POWER, vec![]),
            floor: 0,
        }
    }

    pub fn peek_enemy_next_action(&self, combat: &Combat<R>) -> SkillEffect {
        combat.peek_enemy_next_action()
    }

    pub fn challenge_next_floor(&mut self, rng: &mut R) -> Combat<R> {
        let combat = Combat::new(rng, self.floor, &self.player);

        self.floor += 1;

        if DEBUG {
            println!("floor {}", self.floor);
        }

        combat
    }
}
