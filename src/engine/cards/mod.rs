use super::state::Effect;

#[derive(Clone, Debug)]
pub enum Card {
    Strike,
    Defend,
}

impl Card {
    pub(crate) fn power(&self) -> i32 {
        match self {
            Card::Strike => 1,
            Card::Defend => 1,
        }
    }

    pub(crate) fn effect_to_player(&self) -> Effect {
        match self {
            Card::Strike => Effect { hp: 0, block: 0 },
            Card::Defend => Effect { hp: 0, block: 5 },
        }
    }

    pub(crate) fn effect_to_enemy(&self) -> Effect {
        match self {
            Card::Strike => Effect { hp: -5, block: 0 },
            Card::Defend => Effect { hp: 0, block: 0 },
        }
    }

    // return effect to self and effect to the other
    pub fn into_effect(&self) -> (Effect, Effect) {
        (self.effect_to_player(), self.effect_to_enemy())
    }
}
