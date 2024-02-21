use crate::engine::state::Effect;

use super::{Card, SkillEffect};

#[derive(Debug)]
pub(crate) struct PunchCard;

impl Card for PunchCard {
    fn name(&self) -> &'static str {
        "punch"
    }

    fn power(&self) -> i32 {
        2
    }

    fn effect(&self) -> SkillEffect {
        SkillEffect::Opposite(Effect { hp: -12, block: 0 })
    }
}
