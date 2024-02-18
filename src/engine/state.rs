#[derive(Debug)]
pub(crate) struct CommonState {
    pub(crate) hp: i32,
    pub(crate) block: i32,
}

impl CommonState {
    pub(crate) fn new(hp: i32) -> Self {
        CommonState { hp, block: 0 }
    }

    pub(crate) fn is_dead(&self) -> bool {
        self.hp <= 0
    }

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

pub(crate) struct Effect {
    pub(crate) hp: i32,
    pub(crate) block: i32,
}
