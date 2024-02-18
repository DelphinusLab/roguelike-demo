use std::io::{stdin, stdout, Write};

use colored::{ColoredString, Colorize};
use rand::Rng;
use rand_core::OsRng;
use roguelike_demo::engine::{combat::Combat, Action, Engine, TurnResult};

fn warning<'a>(str: &'a str) -> ColoredString {
    str.on_yellow()
}

fn print_game_info<'a, R: Rng>(floor: usize, combat: &Combat<'a, R>) {
    print!("{} {}\t", "Floor ".yellow(), floor.to_string().yellow());

    println!(
        "{} {}\n",
        "You encounter a(n)".red(),
        combat.enemy.name.red()
    );
    println!(
        "{}",
        format!("Round {}\n", combat.turn.to_string()).on_bright_blue()
    );
}

fn print_hand_of_cards<'a, R: Rng>(combat: &Combat<'a, R>) {
    println!("Your hand of cards:");
    println!("{}", combat.hero.hand);
}

fn print_player_state<'a, R: Rng>(combat: &Combat<'a, R>) {
    println!("{}", format!("{}\t\t{}", "You", combat.hero.state));
}

fn print_enemy_state<'a, R: Rng>(combat: &Combat<'a, R>) {
    println!(
        "{}",
        format!("Enemy({})\t{}", combat.enemy.name, combat.enemy.state)
    );
}

fn print_enemy_next_action<'a, R: Rng>(combat: &Combat<'a, R>) {
    println!(
        "{}",
        warning(&format!(
            "{} will use {} this turn.",
            combat.enemy.name,
            format!("{:?}", combat.enemy.next_action(combat.turn)),
        ))
    );
}

fn print_combat_in_progress<'a, R: Rng>(floor: usize, combat: &Combat<'a, R>) {
    print_game_info(floor, combat);

    print_player_state(combat);
    print_enemy_state(combat);
    print_enemy_next_action(combat);

    print_hand_of_cards(combat);
}

fn main() {
    let mut rng = OsRng;

    let mut engine = Engine::new_game();

    println!("New game");

    loop {
        let floor = engine.floor;
        let mut combat = engine.challenge_next_floor(&mut rng);

        loop {
            let mut player_win = false;

            print_combat_in_progress(floor, &combat);

            print!("Your action(0: end turn, otherwise play card): ");
            let _ = stdout().flush();

            let mut buf = String::new();
            while let Ok(_) = stdin().read_line(&mut buf) {
                println!("{}", buf);
                let card_index: usize = buf.trim().parse().expect("Input not an integer");

                if card_index == 0 {
                    break;
                } else {
                    let result = combat.action(Action::PlayCard(card_index - 1), &mut rng);
                    if result == TurnResult::PlayerWin {
                        println!("You win");

                        player_win = true;
                        break;
                    }

                    print_combat_in_progress(floor, &combat);
                    print!("Your action(0: end turn, otherwise play card): ");
                    let _ = stdout().flush();
                }

                buf = String::new();
            }

            if player_win {
                break;
            }

            let result = combat.action(Action::EndTurn, &mut rng);

            if result == TurnResult::EnemyWin {
                println!("Enemy win");
                return;
            }
        }
    }
}
