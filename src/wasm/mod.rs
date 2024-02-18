use rand::RngCore;
use rand_core::impls;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::engine::{combat::Combat, Action, Engine};

struct TrivialRandom(u64);

impl RngCore for TrivialRandom {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        Ok(self.fill_bytes(dest))
    }
}

static mut RNG: Option<TrivialRandom> = None;
static mut ENGINE: Option<Engine<TrivialRandom>> = None;
static mut COMBAT: Option<Combat<'_, TrivialRandom>> = None;

#[wasm_bindgen]
pub fn new_game() {
    unsafe {
        RNG = Some(TrivialRandom(0));
        ENGINE = Some(Engine::new_game());
        COMBAT = None;
    }
}

#[wasm_bindgen]
pub fn challenge_next_floor() {
    unsafe {
        let combat = ENGINE
            .as_mut()
            .unwrap()
            .challenge_next_floor(RNG.as_mut().unwrap());
        COMBAT = Some(combat);
    }
}

#[wasm_bindgen]
pub fn play_a_card(card_index: usize) {
    unsafe {
        COMBAT
            .as_mut()
            .unwrap()
            .action(Action::PlayCard(card_index), RNG.as_mut().unwrap());
    }
}

#[wasm_bindgen]
pub fn end_turn() {
    unsafe {
        COMBAT
            .as_mut()
            .unwrap()
            .action(Action::EndTurn, RNG.as_mut().unwrap());
    }
}
