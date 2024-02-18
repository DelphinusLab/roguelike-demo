use crate::engine::{cards::SkillEffect, roles::enemy::Monster, state::Effect};

pub(super) struct Robot;

impl Monster for Robot {
    fn hp(&self) -> i32 {
        25
    }

    fn skills(&self) -> Vec<Box<dyn Fn(usize) -> SkillEffect>> {
        vec![
            Box::new(|turn| {
                SkillEffect::Myself(Effect {
                    hp: 0,
                    block: i32::max(5 + turn as i32, 10),
                })
            }),
            Box::new(|turn| {
                SkillEffect::Opposite(Effect {
                    hp: i32::min(-7 - turn as i32, -20),
                    block: 0,
                })
            }),
        ]
    }
}
