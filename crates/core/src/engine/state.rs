use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Effect {
    pub hp: i32,
    pub block: i32,
}

#[derive(Debug)]
pub struct CommonState {
    pub(crate) hp: i32,
    pub(crate) block: i32,
}

impl CommonState {
    pub(crate) fn apply_effect(&mut self, effect: &Effect) {
        self.block += effect.block;

        if effect.hp >= 0 {
            self.hp += effect.hp;
        } else {
            let mut hp = effect.hp.abs();

            if self.block >= hp {
                self.block -= hp;
            } else {
                hp -= self.block;
                self.block = 0;
                self.hp -= hp;
            }
        }
    }
}

impl Display for CommonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("hp: {}, block: {}", self.hp, self.block))
    }
}
