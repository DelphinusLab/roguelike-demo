use rand::Rng;

use crate::{config::DEBUG, engine::cards::SkillEffect};

use super::{
    roles::{
        enemy::{monster::select_an_enemy, Enemy},
        player::{Hero, Player},
        CommonAction,
    },
    Action, TurnResult,
};

pub struct Combat<'a, R: Rng> {
    pub turn: usize,

    pub hero: Hero<'a, R>,
    pub enemy: Enemy,
}

impl<'a, R: Rng> Combat<'a, R> {
    pub(crate) fn new(rng: &mut R, floor: usize, player: &'a Player<R>) -> Combat<'a, R> {
        let mut combat = Combat {
            turn: 0,

            hero: Hero::new(player),
            enemy: select_an_enemy(floor),
        };

        combat.hero.prepare_next_turn(rng);

        combat
    }

    pub(crate) fn peek_enemy_next_action(&self) -> SkillEffect {
        self.enemy.next_action(self.turn)
    }

    fn hint_a_card(&mut self, card_index: usize) -> TurnResult {
        let card = self.hero.hand.get(card_index);
        if card.is_none() {
            return TurnResult::Continue;
        }

        if let Some(card) = card {
            if card.power() > self.hero.state.current_power {
                return TurnResult::Continue;
            }
        }

        let effect = self.hero.attack(card_index);

        match effect {
            SkillEffect::Myself(effect) => self.hero.apply_effect(effect),
            SkillEffect::Opposite(effect) => {
                <Enemy as CommonAction<R>>::apply_effect(&mut self.enemy, effect)
            }
        }

        if <Enemy as CommonAction<R>>::is_dead(&self.enemy) {
            TurnResult::PlayerWin
        } else if self.hero.is_dead() {
            TurnResult::EnemyWin
        } else {
            TurnResult::Continue
        }
    }

    /// Player ends the turn.
    fn end_turn(&mut self, rng: &mut R) -> TurnResult {
        self.hero.end_turn(rng);

        if <Enemy as CommonAction<R>>::is_dead(&self.enemy) {
            return TurnResult::PlayerWin;
        }

        <Enemy as CommonAction<R>>::prepare_next_turn(&mut self.enemy, rng);
        let effect = <Enemy as CommonAction<R>>::attack(&mut self.enemy, self.turn);
        if DEBUG {
            println!("enemy use skill {:?}", effect);
        }

        match effect {
            SkillEffect::Myself(effect) => {
                <Enemy as CommonAction<R>>::apply_effect(&mut self.enemy, effect)
            }
            SkillEffect::Opposite(effect) => self.hero.apply_effect(effect),
        }
        self.enemy.end_turn(rng);

        if self.hero.is_dead() {
            return TurnResult::EnemyWin;
        }

        self.hero.prepare_next_turn(rng);

        self.turn += 1;

        TurnResult::Continue
    }

    pub fn action(&mut self, action: Action, rng: &mut R) -> TurnResult {
        match action {
            Action::PlayCard(card_index) => self.hint_a_card(card_index),
            Action::EndTurn => self.end_turn(rng),
        }
    }
}
