# Tauri + Solid + Typescript

This template should help get you started developing with Tauri, Solid and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


## Setup

1. Prepare sqlite database (Optional)
```bash
cd src-tauri
cargo install sqlx-cli
# Create the database.
sqlx db create
# Run sql migrations
sqlx migrate run
```
2. Run
```bash
pnpm tauri dev
```