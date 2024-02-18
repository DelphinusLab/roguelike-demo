use super::{Enemy, Monster};

use robot::Robot;
use turkey::Turkey;

mod robot;
mod turkey;

lazy_static! {
    static ref MONSTERS: [Box<dyn Monster>; 2] = [Box::new(Robot), Box::new(Turkey)];
}

pub(crate) fn select_an_enemy(floor: usize) -> Enemy {
    let index = floor % MONSTERS.len();

    let monster = MONSTERS[index].as_ref();

    monster.new()
}
