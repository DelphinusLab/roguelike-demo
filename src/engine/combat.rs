use super::roles::enemy::{Enemy, Monster};

pub(crate) struct Combat {
    pub(crate) enemy: Enemy,
    pub(crate) round: usize,
}

impl Combat {
    pub(crate) fn new(floor: usize) -> Combat {
        Combat {
            enemy: Monster::random(floor).into(),
            round: 0,
        }
    }
}
