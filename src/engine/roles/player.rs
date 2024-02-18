use std::marker::PhantomData;

use rand::Rng;

use crate::{
    config::PICK_CARDS_EACH_TURN,
    engine::{
        cards::Card,
        state::{CommonState, Effect},
    },
};

use super::CommonAction;

pub struct Player<R: Rng> {
    pub(crate) common_state: CommonState,

    power: i32,
    asserts: Vec<Card>,

    pub(crate) current_power: i32,
    pub(crate) hand: Vec<Card>,
    candidate_cards: Vec<Card>,
    discard_cards: Vec<Card>,

    _mark: PhantomData<R>,
}

impl<R: Rng> Player<R> {
    pub fn init(hp: i32, power: i32, mut extra_cards: Vec<Card>) -> Self {
        let mut cards = vec![
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Strike,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Defend,
            Card::Defend,
        ];
        cards.append(&mut extra_cards);

        Self {
            common_state: CommonState::new(hp),
            power,
            asserts: cards.clone(),

            current_power: power,
            hand: vec![],
            candidate_cards: cards,
            discard_cards: vec![],

            _mark: PhantomData,
        }
    }

    // for auto test
    #[cfg(test)]
    pub(crate) fn peek_card(&self, card_index: usize) -> &Card {
        self.hand.get(card_index).as_ref().unwrap()
    }

    pub(crate) fn pick_a_card(&mut self, card_index: usize) -> &Card {
        let card = self.hand.remove(card_index);

        assert!(self.current_power >= card.power());
        self.current_power -= card.power();

        self.discard_cards.push(card);

        self.discard_cards.last().unwrap()
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
            self.hand.push(card);
        }
    }

    pub(crate) fn challenge_next_floor(&mut self, rng: &mut R) {
        self.candidate_cards = self.asserts.clone();

        self.discard_cards.clear();
        self.hand.clear();
        self.pick_cards(rng);

        self.common_state.block = 0;
    }

    #[cfg(test)]
    pub(crate) fn number_of_hand(&self) -> usize {
        self.hand.len()
    }
}

impl<R: Rng> CommonAction<R> for Player<R> {
    fn is_dead(&self) -> bool {
        self.common_state.is_dead()
    }

    fn apply_effect(&mut self, effect: Effect) {
        self.common_state.apply_effect(&effect);
    }

    fn end_turn(&mut self, rng: &mut R) {
        self.current_power = self.power;

        self.discard_cards.append(&mut self.hand);
        self.pick_cards(rng);

        self.common_state.block = 0;
    }
}
