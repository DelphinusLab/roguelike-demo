use std::{fmt::Display, rc::Rc};

use crate::{
    config::PICK_CARDS_EACH_TURN,
    engine::{
        cards::{defend::DefendCard, strike::StrikeCard, Card, SkillEffect},
        state::{CommonState, Effect},
    },
};

use super::CommonAction;

#[derive(Debug)]
pub struct Player {
    pub(crate) hp: i32,
    pub(crate) power: i32,
    pub(crate) cards: Vec<Rc<Box<dyn Card>>>,
}

impl Player {
    pub fn init(hp: i32, power: i32, mut extra_cards: Vec<Rc<Box<dyn Card>>>) -> Self {
        let mut cards: Vec<Rc<Box<dyn Card>>> = vec![
            Rc::new(Box::new(StrikeCard)),
            Rc::new(Box::new(StrikeCard)),
            Rc::new(Box::new(StrikeCard)),
            Rc::new(Box::new(StrikeCard)),
            Rc::new(Box::new(StrikeCard)),
            Rc::new(Box::new(DefendCard)),
            Rc::new(Box::new(DefendCard)),
            Rc::new(Box::new(DefendCard)),
            Rc::new(Box::new(DefendCard)),
            Rc::new(Box::new(DefendCard)),
        ];
        cards.append(&mut extra_cards);

        Self { hp, power, cards }
    }
}

#[derive(Debug, Default)]
pub struct HandOfCards(pub(crate) Vec<Rc<Box<dyn Card>>>);

impl HandOfCards {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<Rc<Box<dyn Card>>> {
        self.0.get(index).cloned()
    }
}

impl Display for HandOfCards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, card) in self.0.iter().enumerate() {
            write!(
                f,
                "{}\t{}\t{}\n",
                index + 1,
                format!("{:?}", card),
                card.effect()
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Hero<'a> {
    player: &'a mut Player,

    pub power: i32,
    pub block: i32,
    pub hand: HandOfCards,
    candidate_cards: Vec<Rc<Box<dyn Card>>>,
    discard_cards: Vec<Rc<Box<dyn Card>>>,
}

impl<'a> Hero<'a> {
    pub(crate) fn new(player: &'a mut Player) -> Self {
        let power = player.power;
        let candidate_cards = player.cards.clone();

        Self {
            player,

            block: 0,
            power,
            hand: HandOfCards::default(),
            candidate_cards,
            discard_cards: vec![],
        }
    }

    // pick cards from candidates
    fn pick_cards(&mut self) {
        // FIXME: how to use rng in wasm
        // let mut rng = OsRng::default();

        for _ in 0..PICK_CARDS_EACH_TURN {
            if self.candidate_cards.len() == 0 {
                // SWAP for performance?
                self.candidate_cards.append(&mut self.discard_cards);
            }

            let len = self.candidate_cards.len();

            assert!(len > 0);

            // FIXME: rng.gen_range(0..len)
            let index = 0;

            // LINKED LIST?
            let card = self.candidate_cards.remove(index);
            self.hand.0.push(card);
        }
    }
}

impl<'a> CommonAction for Hero<'a> {
    type Skill = usize;

    fn attack(&mut self, card_index: Self::Skill) -> SkillEffect {
        let card = self.hand.0.remove(card_index);

        assert!(card.power() <= self.power);
        self.power -= card.power();

        let effect = card.effect();

        self.discard_cards.push(card);

        effect
    }

    fn apply_effect(&mut self, effect: Effect) {
        let mut state = CommonState {
            hp: self.player.hp,
            block: self.block,
        };

        state.apply_effect(&effect);

        self.player.hp = state.hp;
        self.block = state.block;
    }

    fn is_dead(&self) -> bool {
        self.player.hp <= 0
    }

    fn prepare_next_turn(&mut self) {
        self.power = self.player.power;
        self.block = 0;

        self.discard_cards.append(&mut self.hand.0);
        self.pick_cards();
    }

    fn end_turn(&mut self) {}
}

impl<'a> Display for Hero<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "hp: {}, block: {}, power: {}",
            self.player.hp, self.block, self.power,
        ))
    }
}
