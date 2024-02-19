use crate::engine::state::Effect;

use super::{Card, SkillEffect};

#[derive(Debug)]
pub(crate) struct StrikeCard;

impl Card for StrikeCard {
    fn power(&self) -> i32 {
        1
    }

    fn effect(&self) -> SkillEffect {
        SkillEffect::Opposite(Effect { hp: -5, block: 0 })
    }
}
