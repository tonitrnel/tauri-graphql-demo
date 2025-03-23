use crate::models::todo::Todo;

use super::context::Context;
use super::{relay, scalar};
use juniper::{graphql_object, EmptySubscription, Executor, FieldResult, RootNode};

pub struct Query;

#[graphql_object]
#[graphql(context = Context, scalar = scalar::CustomScalarValue)]
impl Query {
    pub fn greet(name: String) -> String {
        format!("Hello, {}! You've been greeted from Rust!", name)
    }
    pub async fn list_todos(
        executor: &Executor<'_, '_, Context, scalar::CustomScalarValue>,
        ctx: &Context,
        first: Option<i32>,
        after: Option<relay::Cursor>,
        last: Option<i32>,
        before: Option<relay::Cursor>,
    ) -> FieldResult<relay::Connection<Todo>> {
        let patination = relay::Pagination {
            first,
            after,
            last,
            before,
        };
        let conn = relay::Connection::new(
            executor,
            patination,
            async |pag| ctx.todo_repo.list_todos(&pag).await,
            async || ctx.todo_repo.total().await,
        )
        .await?;
        Ok(conn)
    }
}

pub struct Mutation;
#[graphql_object]
#[graphql(context = Context, scalar = scalar::CustomScalarValue)]
impl Mutation {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    pub async fn add_todo(ctx: &Context, description: String) -> FieldResult<scalar::ID> {
        let id = ctx.todo_repo.add_todo(description).await?;
        Ok(id)
    }
    pub async fn complete_todo(ctx: &Context, id: scalar::ID, done: bool) -> FieldResult<bool> {
        let suc = ctx.todo_repo.complete_todo(id, done).await?;
        Ok(suc)
    }
    pub async fn remove_todo(ctx: &Context, id: scalar::ID) -> FieldResult<bool> {
        let suc = ctx.todo_repo.remove_todo(id).await?;
        Ok(suc)
    }
    pub async fn edit_todo(
        ctx: &Context,
        id: scalar::ID,
        description: String,
    ) -> FieldResult<bool> {
        let suc = ctx.todo_repo.edit_todo(id, description).await?;
        Ok(suc)
    }
    pub async fn toggle_all(ctx: &Context, done: bool) -> FieldResult<bool> {
        let suc = ctx.todo_repo.toggle_all(done).await?;
        Ok(suc)
    }
    pub async fn clear_completed(ctx: &Context) -> FieldResult<bool> {
        let suc = ctx.todo_repo.clear_completed().await?;
        Ok(suc)
    }
}

pub type Schema =
    RootNode<'static, Query, Mutation, EmptySubscription<Context>, scalar::CustomScalarValue>;

pub fn create_schema() -> Schema {
    let schema = Schema::new_with_scalar_value(Query, Mutation, EmptySubscription::new());
    #[cfg(debug_assertions)]
    {
        // 每次启动时输出 schema 到文件
        let root_dir = std::env::current_dir().unwrap();
        let path = root_dir.join("graphql.schema");
        std::fs::write(&path, schema.as_sdl()).unwrap();
    }
    schema
}
