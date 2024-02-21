use crate::engine::state::Effect;

use super::{Card, SkillEffect};

#[derive(Debug)]
pub(crate) struct HealCard;

impl Card for HealCard {
    fn name(&self) -> &'static str {
        "heal"
    }

    fn power(&self) -> i32 {
        1
    }

    fn effect(&self) -> SkillEffect {
        SkillEffect::Myself(Effect { hp: 5, block: 0 })
    }
}
