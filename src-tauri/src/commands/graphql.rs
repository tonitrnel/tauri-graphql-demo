use crate::graphql::{self, scalar};
use crate::state::AppState;
use juniper::http::GraphQLRequest;
use tauri::command;

#[command]
pub async fn graphql(
    state: tauri::State<'_, AppState>,
    body: GraphQLRequest<scalar::CustomScalarValue>,
) -> Result<serde_json::Value, serde_json::Value> {
    let pool = state.pool.clone();
    let context = graphql::Context::new(pool);

    let response = body.execute(&state.schema, &context).await;
    match (response.is_ok(), serde_json::to_value(response)) {
        (true, Ok(v)) => Ok(v),
        (false, Ok(v)) => Err(v),
        (_, Err(e)) => Err(serde_json::Value::String(e.to_string())),
    }
}
