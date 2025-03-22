use tauri::Manager;
use dotenvy::dotenv;

mod commands;
mod graphql;
mod state;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> anyhow::Result<()> {
    dotenv().unwrap();
    let pool = sqlx::SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?;
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(state::build_app_state(pool));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::graphql::graphql])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
