use crate::engine::{cards::SkillEffect, roles::enemy::Monster, state::Effect};

pub(super) struct Turkey;

impl Monster for Turkey {
    fn name(&self) -> &'static str {
        "turkey"
    }

    fn hp(&self) -> i32 {
        15
    }

    fn skills(&self) -> Vec<Box<dyn Fn(usize) -> SkillEffect>> {
        vec![Box::new(|turn| {
            SkillEffect::Opposite(Effect {
                hp: i32::min(-15 - turn as i32, -20),
                block: 0,
            })
        })]
    }
}
