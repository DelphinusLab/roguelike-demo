use rand::{
    rngs::{OsRng, ThreadRng},
    thread_rng, Rng,
};

use crate::engine::{Action, Engine, TurnResult};

#[test]
fn test() {
    let mut engine = Engine::init();

    let mut rng = thread_rng();

    loop {
        engine.challenge_next_floor(&mut rng);

        let mut retry = 3;
        let mut player_win = false;
        while retry > 0 {
            let card_index = rng.gen_range(0..engine.player.number_of_hand());
            let card = engine.player.peek_card(card_index);

            if engine.player.current_power >= card.power() {
                let result = engine.action(Action::PlayCard(card_index), &mut rng);

                match result {
                    TurnResult::PlayerWin => {
                        player_win = true;
                        break;
                    }
                    TurnResult::EnemyWin => {
                        println!("enemy win");

                        return;
                    }
                    TurnResult::Continue => (),
                }
            }

            retry -= 1;
        }

        if player_win {
            continue;
        }

        let result = engine.action(Action::EndTurn, &mut rng);

        if result == TurnResult::EnemyWin {
            println!("enemy win");

            return;
        }
    }
}
