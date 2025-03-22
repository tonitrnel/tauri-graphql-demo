use sqlx::SqlitePool;

pub struct Context {
    // 这里除了存储数据库连接外还用于存储 service 或 repository
    #[allow(unused)]
    pool: SqlitePool
  }
  
  impl Context{
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
  }
  
  impl juniper::Context for Context {}