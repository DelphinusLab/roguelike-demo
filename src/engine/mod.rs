use rand::Rng;

use self::roles::enemy::Enemy;
use self::{combat::Combat, roles::player::Player};
use crate::config::{DEBUG, DEFAULT_HP, DEFAULT_POWER};
use crate::engine::roles::CommonAction;

mod cards;
mod combat;
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
    floor: usize,

    combat: Option<Combat>,
}

impl<R: Rng> Engine<R> {
    pub fn init() -> Self {
        Self {
            player: Player::init(DEFAULT_HP, DEFAULT_POWER, vec![]),
            floor: 0,

            combat: None,
        }
    }

    pub fn challenge_next_floor(&mut self, rng: &mut R) {
        println!("challenge_next_floor",);
        self.player.challenge_next_floor(rng);
        self.floor += 1;
        self.combat = Some(Combat::new(self.floor));

        if DEBUG {
            println!("floor {}", self.floor);
            println!("state: {:?}", self.player.common_state);
            println!("power: {}", self.player.current_power);
            println!("cards: {:?}", self.player.hand);
            println!("enemy: {:?}", self.combat.as_ref().unwrap().enemy);
        }
    }

    // Apply a selected card
    fn play_a_card(&mut self, card_index: usize) -> TurnResult {
        let card = self.player.pick_a_card(card_index);

        if DEBUG {
            println!("player pick a card: {:?}", card);
        }

        let (effect_to_self, effect_to_enemy) = card.into_effect();

        self.player.apply_effect(effect_to_self);

        let mut enemy = &mut self.combat.as_mut().unwrap().enemy;
        <Enemy as CommonAction<R>>::apply_effect(&mut enemy, effect_to_enemy);

        if <Enemy as CommonAction<R>>::is_dead(&mut enemy) {
            TurnResult::PlayerWin
        } else {
            TurnResult::Continue
        }
    }

    fn enemy_attack(&mut self, rng: &mut R) -> TurnResult {
        let round = self.combat.as_ref().unwrap().round;

        let combat = self.combat.as_mut().unwrap();
        let (effect_to_self, effect_to_enemy) = combat.enemy.attack(round);

        self.player.apply_effect(effect_to_enemy);
        <Enemy as CommonAction<R>>::apply_effect(&mut combat.enemy, effect_to_self);
        combat.enemy.end_turn(rng);

        if self.player.is_dead() {
            TurnResult::EnemyWin
        } else {
            TurnResult::Continue
        }
    }

    fn end_turn(&mut self, rng: &mut R) -> TurnResult {
        if DEBUG {
            println!("end turn");
        }

        self.player.end_turn(rng);

        self.enemy_attack(rng)
    }

    pub fn action(&mut self, action: Action, rng: &mut R) -> TurnResult {
        match action {
            Action::PlayCard(card_index) => self.play_a_card(card_index),
            Action::EndTurn => self.end_turn(rng),
        }
    }
}
