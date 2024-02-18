use crate::{config::DEBUG, engine::cards::SkillEffect};

use super::{
    roles::{
        enemy::{monster::select_an_enemy, Enemy},
        player::{Hero, Player},
        CommonAction,
    },
    Action, TurnResult,
};

pub struct Combat<'a> {
    pub turn: usize,

    pub hero: Hero<'a>,
    pub enemy: Enemy,
}

impl<'a> Combat<'a> {
    pub(crate) fn new(floor: usize, player: &'a mut Player) -> Combat<'a> {
        let mut combat = Combat {
            turn: 0,

            hero: Hero::new(player),
            enemy: select_an_enemy(floor),
        };

        combat.hero.prepare_next_turn();

        combat
    }

    pub fn peek_enemy_next_action(&self) -> SkillEffect {
        self.enemy.next_action(self.turn)
    }

    fn hint_a_card(&mut self, card_index: usize) -> TurnResult {
        let card = self.hero.hand.get(card_index);
        if card.is_none() {
            return TurnResult::Continue;
        }

        if let Some(card) = card {
            if card.power() > self.hero.power {
                return TurnResult::Continue;
            }
        }

        let effect = self.hero.attack(card_index);

        match effect {
            SkillEffect::Myself(effect) => self.hero.apply_effect(effect),
            SkillEffect::Opposite(effect) => self.enemy.apply_effect(effect),
        }

        if self.enemy.is_dead() {
            TurnResult::PlayerWin
        } else if self.hero.is_dead() {
            TurnResult::EnemyWin
        } else {
            TurnResult::Continue
        }
    }

    /// Player ends the turn.
    fn end_turn(&mut self) -> TurnResult {
        self.hero.end_turn();

        if self.enemy.is_dead() {
            return TurnResult::PlayerWin;
        }

        self.enemy.prepare_next_turn();
        let effect = self.enemy.attack(self.turn);
        if DEBUG {
            println!("enemy use skill {:?}", effect);
        }

        match effect {
            SkillEffect::Myself(effect) => self.enemy.apply_effect(effect),
            SkillEffect::Opposite(effect) => self.hero.apply_effect(effect),
        }
        self.enemy.end_turn();

        if self.hero.is_dead() {
            return TurnResult::EnemyWin;
        }

        self.hero.prepare_next_turn();

        self.turn += 1;

        TurnResult::Continue
    }

    pub fn action(&mut self, action: Action) -> TurnResult {
        match action {
            Action::PlayCard(card_index) => self.hint_a_card(card_index),
            Action::EndTurn => self.end_turn(),
        }
    }
}
