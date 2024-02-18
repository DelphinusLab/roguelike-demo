use std::fmt::Debug;

use rand::Rng;

use crate::engine::{
    cards::SkillEffect,
    state::{CommonState, Effect},
};

use super::CommonAction;

pub(crate) mod monster;

pub(crate) trait Monster: Sync + Send {
    fn name(&self) -> &'static str;
    fn hp(&self) -> i32;
    fn skills(&self) -> Vec<Box<dyn Fn(usize) -> SkillEffect>>;

    fn new(&self) -> Enemy {
        Enemy {
            name: self.name(),
            state: CommonState {
                hp: self.hp(),
                block: 0,
            },
            actions: self.skills(),
        }
    }
}

pub struct Enemy {
    pub name: &'static str,
    pub state: CommonState,
    pub(crate) actions: Vec<Box<dyn Fn(usize) -> SkillEffect>>,
}

impl Debug for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Enemy").field("state", &self.state).finish()
    }
}

impl Enemy {
    pub fn next_action(&self, turn: usize) -> SkillEffect {
        self.actions[turn % self.actions.len()](turn)
    }
}

impl<R: Rng> CommonAction<R> for Enemy {
    // depends on round
    type Skill = usize;

    fn attack(&mut self, turn: Self::Skill) -> SkillEffect {
        self.next_action(turn)
    }

    fn apply_effect(&mut self, effect: Effect) {
        self.state.apply_effect(&effect)
    }

    fn is_dead(&self) -> bool {
        self.state.hp <= 0
    }

    fn prepare_next_turn(&mut self, _rng: &mut R) {
        self.state.block = 0;
    }

    fn end_turn(&mut self, _rng: &mut R) {}
}
