use std::fmt::Debug;

use super::state::Effect;

pub(crate) mod defend;
pub(crate) mod strike;

#[derive(Debug)]
pub enum SkillEffect {
    Myself(Effect),
    Opposite(Effect),
}

pub(crate) trait Card: Debug {
    fn power(&self) -> i32;
    fn effect(&self) -> SkillEffect;
}
