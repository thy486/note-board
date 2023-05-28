#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod ipc;
mod traits;
mod wits;
use traits::*;

fn main() {
    tauri::Builder::default()
        .nest::<ipc::Ipc>()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
