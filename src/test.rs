#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use crate::engine::{Action, Engine, TurnResult};

    #[test]
    fn test() {
        let mut engine = Engine::init();

        let mut rng = thread_rng();

        loop {
            let mut combat = engine.challenge_next_floor(&mut rng);

            loop {
                println!("player\n{:?}", combat.hero.state);
                println!("enemy\n{:?}", combat.enemy);

                let mut retry = 3;
                let mut player_win = false;
                while retry > 0 {
                    let card_index = rng.gen_range(0..combat.hero.hand.len());
                    let card = combat.hero.hand.get(card_index).unwrap();

                    if combat.hero.state.current_power >= card.power() {
                        println!("play a card: {:?}", card);

                        let result = combat.action(Action::PlayCard(card_index), &mut rng);

                        match result {
                            TurnResult::PlayerWin => {
                                player_win = true;
                                break;
                            }
                            TurnResult::EnemyWin => {
                                println!("enemy win, floor: {}", engine.floor);

                                return;
                            }
                            TurnResult::Continue => (),
                        }
                    }

                    retry -= 1;
                }

                if player_win {
                    break;
                }

                let result = combat.action(Action::EndTurn, &mut rng);

                if result == TurnResult::EnemyWin {
                    println!("enemy win, floor: {}", engine.floor);

                    return;
                }
            }
        }
    }
}
