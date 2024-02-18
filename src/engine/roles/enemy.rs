use num_traits::FromPrimitive;
use rand::Rng;

use crate::engine::state::{CommonState, Effect};

use super::CommonAction;

const KINDS_OF_MONSTER: usize = 1;

#[derive(Debug, FromPrimitive)]
pub enum Monster {
    Robot,
}

impl Monster {
    fn new_state(&self) -> CommonState {
        match self {
            Monster::Robot => CommonState { hp: 10, block: 10 },
        }
    }
}

impl Monster {
    pub(crate) fn random(floor: usize) -> Monster {
        Monster::from_usize(floor % KINDS_OF_MONSTER).unwrap()
    }
}

#[derive(Debug)]
pub(crate) struct Enemy {
    pub(crate) state: CommonState,
    monster: Monster,
}

impl Enemy {
    // effect to self, effect to player
    pub(crate) fn attack(&self, round: usize) -> (Effect, Effect) {
        match self.monster {
            Monster::Robot => (Effect { hp: 0, block: 10 }, Effect { hp: -5, block: 0 }),
        }
    }
}

impl<R: Rng> CommonAction<R> for Enemy {
    fn is_dead(&self) -> bool {
        self.state.hp <= 0
    }

    fn apply_effect(&mut self, effect: Effect) {
        self.state.apply_effect(&effect)
    }

    fn end_turn(&mut self, rng: &mut R) {
        self.state.block = 0;
    }
}

impl From<Monster> for Enemy {
    fn from(monster: Monster) -> Self {
        match monster {
            Monster::Robot => Self {
                state: monster.new_state(),
                monster: Monster::Robot,
            },
        }
    }
}
