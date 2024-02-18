use rand::Rng;

use super::{cards::SkillEffect, state::Effect};

pub(crate) mod enemy;
pub(crate) mod player;

pub(crate) trait CommonAction<R: Rng> {
    type Skill;

    fn attack(&mut self, skill: Self::Skill) -> SkillEffect;
    fn apply_effect(&mut self, effect: Effect);

    fn is_dead(&self) -> bool;
    fn prepare_next_turn(&mut self, rng: &mut R);
    fn end_turn(&mut self, rng: &mut R);
}
