use anyhow::Ok;

use crate::{
    graphql::{relay, scalar},
    models::todo::Todo,
};

pub struct TodoRepository {
    pool: sqlx::SqlitePool,
}

impl TodoRepository {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
    pub async fn add_todo(&self, description: String) -> anyhow::Result<scalar::ID> {
        let mut conn = self.pool.acquire().await?;

        let id = sqlx::query!(
            r#"
            INSERT INTO todos ( description )
            VALUES ( ?1 )
            "#,
            description
        )
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();
        Ok(id.into())
    }

    pub async fn complete_todo(&self, id: scalar::ID, done: bool) -> anyhow::Result<bool> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE todos
            SET done = ?2
            WHERE id = ?1
            "#,
            id,
            done
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    pub async fn toggle_all(&self, done: bool) -> anyhow::Result<bool> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE todos
            SET done = ?1
            WHERE done <> ?1
            "#,
            done
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    pub async fn remove_todo(&self, id: scalar::ID) -> anyhow::Result<bool> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM todos WHERE id = ?1
            "#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    pub async fn clear_completed(&self) -> anyhow::Result<bool> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM todos WHERE done = TRUE
            "#,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    pub async fn edit_todo(&self, id: scalar::ID, description: String) -> anyhow::Result<bool> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE todos
            SET description = ?2
            WHERE id = ?1
            "#,
            id,
            description
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    pub async fn list_todos(&self, pag: &relay::Pagination) -> anyhow::Result<Vec<Todo>> {
        use sqlx::Arguments;
        use std::fmt::Write;

        let mut query = String::from("SELECT id, description, done, created_at FROM todos ");
        let mut arguments = sqlx::sqlite::SqliteArguments::default();

        if let Some(after) = pag.after.as_ref() {
            write!(
                query,
                "WHERE (id, created_at) > ( ?{}, ?{} ) ",
                arguments.len() + 1,
                arguments.len() + 2
            )?;
            arguments.add(after.id).unwrap();
            arguments.add(after.created_at).unwrap();
        } else if let Some(before) = pag.before.as_ref() {
            write!(
                query,
                "WHERE (id, created_at) < ( ?{}, ?{} ) ",
                arguments.len() + 1,
                arguments.len() + 2
            )?;
            arguments.add(before.id).unwrap();
            arguments.add(before.created_at).unwrap();
        }
        if pag.last.is_some() {
            query.push_str("ORDER BY id DESC, created_at DESC ");
        } else {
            query.push_str("ORDER BY id ASC, created_at ASC ");
        }
        write!(query, "LIMIT ?{}", arguments.len() + 1)?;
        arguments.add(pag.limit()).unwrap();

        let mut query = sqlx::QueryBuilder::<sqlx::Sqlite>::with_arguments(query, arguments);
        let recs = query
            .build_query_as::<Todo>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                println!("Error: {:?}", e);
                println!("SQL: \n{}", query.sql());
                e
            })?;
        Ok(recs)
    }

    pub async fn total(&self) -> anyhow::Result<i32> {
        let rec = sqlx::query!("SELECT COUNT(*) as total FROM todos")
        .fetch_one(&self.pool)
        .await?;
        Ok(rec.total as i32)
    }
}
