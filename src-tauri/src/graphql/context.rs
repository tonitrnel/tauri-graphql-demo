use sqlx::SqlitePool;

use crate::repositories::TodoRepository;

pub struct Context {
    pub todo_repo: TodoRepository,
}

impl Context {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
          todo_repo: TodoRepository::new(pool),
        }
    }
}

impl juniper::Context for Context {}
