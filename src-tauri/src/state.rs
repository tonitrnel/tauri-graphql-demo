use sqlx::SqlitePool;
use crate::graphql;

pub struct AppState {
    pub pool: SqlitePool,
    pub schema: graphql::Schema
}

pub fn build_app_state(pool: SqlitePool) -> AppState{
    AppState {
        pool,
        schema: graphql::create_schema(),
    }
}