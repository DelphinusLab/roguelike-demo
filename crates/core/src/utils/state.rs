use serde::Serialize;

use crate::engine::{cards::SkillEffect, combat::Combat};

#[derive(Serialize)]
pub struct Card {
    name: &'static str,
    skill: SkillEffect,
    power: i32,
}

#[derive(Serialize)]
pub struct GameState {
    floor: usize,
    turn: usize,

    hero_hp: i32,
    hero_power: i32,
    hero_block: i32,
    hand_of_card: Vec<Card>,

    enemy_name: &'static str,
    enemy_hp: i32,
    enemy_block: i32,
    enemy_action: SkillEffect,
}

impl GameState {
    pub fn from(floor: usize, combat: &Combat) -> GameState {
        GameState {
            floor,
            turn: combat.turn,

            hero_hp: combat.hero.player.hp,
            hero_power: combat.hero.power,
            hero_block: combat.hero.block,
            hand_of_card: combat
                .hero
                .hand
                .0
                .iter()
                .map(|card| Card {
                    name: card.name(),
                    skill: card.effect(),
                    power: card.power(),
                })
                .collect(),
            enemy_name: combat.enemy.name,
            enemy_hp: combat.enemy.state.hp,
            enemy_block: combat.enemy.state.block,
            enemy_action: combat.peek_enemy_next_action(),
        }
    }
}
