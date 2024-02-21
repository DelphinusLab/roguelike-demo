use std::fmt::{Debug, Display};

use serde::Serialize;

use super::state::Effect;

pub(crate) mod defend;
pub(crate) mod heal;
pub(crate) mod punch;
pub(crate) mod strike;

#[derive(Debug, Serialize)]
pub enum SkillEffect {
    Myself(Effect),
    Opposite(Effect),
}

impl SkillEffect {
    pub fn to_myself(&self) -> bool {
        match self {
            SkillEffect::Myself(_) => true,
            SkillEffect::Opposite(_) => false,
        }
    }

    pub fn to_opposite(&self) -> bool {
        match self {
            SkillEffect::Myself(_) => false,
            SkillEffect::Opposite(_) => true,
        }
    }
}

impl Display for SkillEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

pub trait Card: Debug {
    fn name(&self) -> &'static str;
    fn power(&self) -> i32;
    fn effect(&self) -> SkillEffect;
}
