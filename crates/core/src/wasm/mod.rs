use crate::engine::roles::CommonAction;
use wasm_bindgen::prelude::wasm_bindgen;
use zkwasm_rust_sdk::require;
use zkwasm_rust_sdk::jubjub::BabyJubjubPoint;
use zkwasm_rust_sdk::jubjub::JubjubSignature;
use zkwasm_rust_sdk::wasm_input;
use primitive_types::U256;
use sha2::{Sha256, Digest};


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

/*
#[inline(always)]
fn public_input() -> u64 {
    unsafe { wasm_input(1) }
}
*/

#[inline(always)]
fn private_input() -> u64 {
    unsafe { wasm_input(0) }
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

fn step_command(command: u64) {
    assert_hero_alive();
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

#[wasm_bindgen]
pub fn zkmain() {
    let mut hasher = Sha256::new();

    new_game();

    let commands_len = private_input();
    challenge_next_floor();

    for _ in 0..commands_len {
        let command = private_input();

        hasher.update(command.to_le_bytes());

        step_command(command);
    }

    assert_game_finish();

    let msghash = hasher.finalize();

    zkwasm_rust_sdk::dbg!("command hash is {:?}\n", msghash);

    /*
    let msghash = unsafe {[
        wasm_input(1),
        wasm_input(1),
        wasm_input(1),
        wasm_input(1),
    ]};
    */

    zkwasm_rust_sdk::dbg!("msg {:?}\n", msghash);

    let pk = unsafe {BabyJubjubPoint {
        x: U256([
                wasm_input(0),
                wasm_input(0),
                wasm_input(0),
                wasm_input(0),
        ]),
        y: U256([
                wasm_input(0),
                wasm_input(0),
                wasm_input(0),
                wasm_input(0),
        ]),
    }};
    zkwasm_rust_sdk::dbg!("process sig\n");

    let sig = unsafe {JubjubSignature {
        sig_r: BabyJubjubPoint {
            x: U256([
                    wasm_input(0),
                    wasm_input(0),
                    wasm_input(0),
                    wasm_input(0),
            ]),
            y: U256([
                    wasm_input(0),
                    wasm_input(0),
                    wasm_input(0),
                    wasm_input(0),
            ]),
        },
        sig_s: [
            wasm_input(0),
            wasm_input(0),
            wasm_input(0),
            wasm_input(0),
        ]
    }};
    zkwasm_rust_sdk::dbg!("start verifying ...\n");

    let msghash_u64 = [
        u64::from_be_bytes(msghash[24..32].try_into().unwrap()),
        u64::from_be_bytes(msghash[16..24].try_into().unwrap()),
        u64::from_be_bytes(msghash[8..16].try_into().unwrap()),
        u64::from_be_bytes(msghash[0..8].try_into().unwrap()),
    ];

    sig.verify(&pk, &msghash_u64);
}
