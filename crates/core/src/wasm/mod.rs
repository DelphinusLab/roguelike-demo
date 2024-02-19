use wasm_bindgen::prelude::wasm_bindgen;

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
