#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod hanoi;
mod permutation;

use state::state_setup;
use state::{PermutationState, HanoiState};
use tauri::AppHandle;
use tauri::State;

#[tauri::command]
async fn hanoi(app: AppHandle, state: State<'_, HanoiState>) -> Result<(), String> {
    hanoi::generate_hanoi_animation(
        state.rods_number.lock().unwrap().to_owned(),
        state.plates_number.lock().unwrap().to_owned(),
        state.fps.lock().unwrap().to_owned(),
        app
    ).map_err(|e| e.to_string())
}

#[tauri::command]
async fn permutation(app: AppHandle, state: State<'_, PermutationState>) -> Result<(), String> {
    state.permutation_sequence.lock().unwrap().clear();
    let mut count = 0;
    permutation::permutation_string(
        state.content.lock().unwrap().clone(),
        state.k.lock().unwrap().to_owned(),
        state.n.lock().unwrap().to_owned(),
        &mut count,
        app
    ).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| Ok(state_setup(app.handle())))
        .manage(PermutationState::default())
        .manage(HanoiState::default())
        .invoke_handler(tauri::generate_handler![
            permutation,
            hanoi
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}