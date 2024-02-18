use std::{fmt::Display, marker::PhantomData};

use crate::{
    config::PICK_CARDS_EACH_TURN,
    engine::{
        cards::{defend::DefendCard, strike::StrikeCard, Card, SkillEffect},
        state::{CommonState, Effect},
    },
};
use colored::Colorize;
use rand::Rng;

use super::CommonAction;

#[derive(Debug)]
pub struct Player<R: Rng> {
    pub(crate) hp: i32,
    pub(crate) power: i32,
    pub(crate) cards: Vec<Box<dyn Card>>,

    _mark: PhantomData<R>,
}

impl<R: Rng> Player<R> {
    pub fn init(hp: i32, power: i32, mut extra_cards: Vec<Box<dyn Card>>) -> Self {
        let mut cards: Vec<Box<dyn Card>> = vec![
            Box::new(StrikeCard),
            Box::new(StrikeCard),
            Box::new(StrikeCard),
            Box::new(StrikeCard),
            Box::new(StrikeCard),
            Box::new(DefendCard),
            Box::new(DefendCard),
            Box::new(DefendCard),
            Box::new(DefendCard),
            Box::new(DefendCard),
        ];
        cards.append(&mut extra_cards);

        Self {
            hp,
            power,
            cards,

            _mark: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct PlayerState {
    pub(crate) common: CommonState,
    pub(crate) current_power: i32,
}

impl Display for PlayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "hp: {}, block: {}, power: {}",
            self.common.hp, self.common.block, self.current_power,
        ))
    }
}

#[derive(Debug, Default)]
pub struct HandOfCards<'a>(pub(crate) Vec<&'a Box<dyn Card>>);

impl<'a> HandOfCards<'a> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&'a Box<dyn Card>> {
        self.0.get(index).copied()
    }
}

impl<'a> Display for HandOfCards<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, card) in self.0.iter().enumerate() {
            let effect = card.effect();

            let to_myself = effect.to_myself();

            write!(
                f,
                "{}\t{}\t{}\n",
                index + 1,
                if to_myself {
                    format!("{:?}", card).green()
                } else {
                    format!("{:?}", card).red()
                },
                effect
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Hero<'a, R: Rng> {
    player: &'a Player<R>,

    pub state: PlayerState,
    pub hand: HandOfCards<'a>,
    candidate_cards: Vec<&'a Box<dyn Card>>,
    discard_cards: Vec<&'a Box<dyn Card>>,
}

impl<'a, R: Rng> Hero<'a, R> {
    pub(crate) fn new(player: &'a Player<R>) -> Self {
        Self {
            player,

            state: PlayerState {
                common: CommonState {
                    hp: player.hp,
                    block: 0,
                },
                current_power: player.power,
            },
            hand: HandOfCards::default(),
            candidate_cards: player.cards.iter().collect(),
            discard_cards: vec![],
        }
    }

    // pick cards from candidates
    fn pick_cards(&mut self, rng: &mut R) {
        for _ in 0..PICK_CARDS_EACH_TURN {
            if self.candidate_cards.len() == 0 {
                // SWAP for performance?
                self.candidate_cards.append(&mut self.discard_cards);
            }

            let len = self.candidate_cards.len();

            assert!(len > 0);

            let index = rng.gen_range(0..len);
            // LINKED LIST?
            let card = self.candidate_cards.remove(index);
            self.hand.0.push(card);
        }
    }
}

impl<'a, R: Rng> CommonAction<R> for Hero<'a, R> {
    type Skill = usize;

    fn attack(&mut self, card_index: Self::Skill) -> SkillEffect {
        let card = self.hand.0.remove(card_index);

        assert!(card.power() <= self.state.current_power);
        self.state.current_power -= card.power();

        self.discard_cards.push(card);

        card.effect()
    }

    fn apply_effect(&mut self, effect: Effect) {
        self.state.common.apply_effect(&effect);
    }

    fn is_dead(&self) -> bool {
        self.state.common.hp <= 0
    }

    fn prepare_next_turn(&mut self, rng: &mut R) {
        self.state.current_power = self.player.power;
        self.state.common.block = 0;

        self.discard_cards.append(&mut self.hand.0);
        self.pick_cards(rng);
    }

    fn end_turn(&mut self, _rng: &mut R) {}
}
