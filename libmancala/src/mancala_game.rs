use super::PIT_LAST;
use crate::Board;
use crate::StartPit;
use crate::ID;

use std::sync::Mutex;

static BOARD: Mutex<Board> = Mutex::new(Board::new());

#[no_mangle]
extern "C" fn start_game() {
    *BOARD.lock().unwrap() = Board::new()
}

#[no_mangle]
extern "C" fn game_state() {
    //*BOARD.lock().unwrap() = Board::new()
}

#[no_mangle]
extern "C" fn take_turn(id: ID, start_pit: StartPit) {
    BOARD.lock().unwrap().turn_avalanche(&id, &start_pit)
}

#[no_mangle]
extern "C" fn marble_count(index: f64) -> f64 {
    BOARD.lock().unwrap().marble_count(index as usize).into()
}

#[no_mangle]
extern "C" fn p1_count() -> f64 {
    BOARD.lock().unwrap().marble_count(0).into()
}

#[no_mangle]
extern "C" fn p2_count() -> f64 {
    BOARD.lock().unwrap().marble_count(PIT_LAST).into()
}
