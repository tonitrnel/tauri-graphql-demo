// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    tauri_graphql_demo_lib::run()
        .await
        .unwrap_or_else(|e| panic!("Error run tauri application {e:?}"));
}
