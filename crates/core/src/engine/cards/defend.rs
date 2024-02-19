use crate::engine::state::Effect;

use super::{Card, SkillEffect};

#[derive(Debug)]
pub(crate) struct DefendCard;

impl Card for DefendCard {
    fn name(&self) -> &'static str {
        "defend"
    }

    fn power(&self) -> i32 {
        1
    }

    fn effect(&self) -> SkillEffect {
        SkillEffect::Myself(Effect { hp: 0, block: 5 })
    }
}
