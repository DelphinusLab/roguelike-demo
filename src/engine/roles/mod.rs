use rand::Rng;

use super::state::Effect;

pub(crate) mod enemy;
pub(crate) mod player;

pub(crate) trait CommonAction<R: Rng> {
    fn is_dead(&self) -> bool;

    fn apply_effect(&mut self, effect: Effect);

    fn end_turn(&mut self, rng: &mut R);
}
