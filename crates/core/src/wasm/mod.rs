use crate::engine::roles::CommonAction;
use wasm_bindgen::prelude::wasm_bindgen;
use zkwasm_rust_sdk::{require, wasm_input};

use crate::{
    engine::{combat::Combat, Action, Engine},
    utils::state::GameState,
};

static mut ENGINE: Option<Engine> = None;
static mut COMBAT: Option<Combat<'_>> = None;

#[wasm_bindgen]
pub fn new_game() {
    unsafe {
        ENGINE = Some(Engine::new_game());
        COMBAT = None;
    }
}

#[wasm_bindgen]
pub fn challenge_next_floor() {
    unsafe {
        let combat = ENGINE.as_mut().unwrap().challenge_next_floor();
        COMBAT = Some(combat);
    }
}

#[wasm_bindgen]
pub fn play_a_card(card_index: usize) {
    unsafe {
        COMBAT
            .as_mut()
            .unwrap()
            .action(Action::PlayCard(card_index));
    }
}

#[wasm_bindgen]
pub fn end_turn() {
    unsafe {
        COMBAT.as_mut().unwrap().action(Action::EndTurn);
    }
}

#[wasm_bindgen]
pub fn state() -> String {
    let state =
        unsafe { GameState::from(ENGINE.as_ref().unwrap().floor, &COMBAT.as_ref().unwrap()) };

    serde_json::to_string_pretty(&state).unwrap()
}

#[inline(always)]
fn public_input() -> u64 {
    unsafe { wasm_input(1) }
}

#[inline(always)]
fn assert_hero_alive() {
    unsafe { require(!COMBAT.as_ref().unwrap().hero.is_dead()) }
}

#[inline(always)]
fn assert_game_finish() {
    unsafe { require(ENGINE.as_ref().unwrap().player.is_dead()) }
}

const COMMAND_END_TURN: u64 = 0u64;

#[wasm_bindgen]
pub fn zkmain() {
    new_game();

    let command_len = public_input();
    challenge_next_floor();

    for _ in 0..command_len {
        assert_hero_alive();

        let command = public_input();

        if command == COMMAND_END_TURN {
            end_turn();
        } else {
            play_a_card(command as usize - 1);
        }

        unsafe {
            if COMBAT.as_ref().unwrap().enemy.is_dead() {
                challenge_next_floor();
            }
        }
    }

    assert_game_finish();
}
