use crate::graphql::{self, relay, scalar};
use juniper::graphql_object;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Todo {
    id: scalar::ID,
    description: String,
    done: bool,
    created_at: scalar::Timestamp,
}

#[graphql_object(context = graphql::Context, scalar = graphql::CustomScalarValue)]
impl Todo {
    pub fn id(&self) -> &scalar::ID {
        &self.id
    }
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn done(&self) -> bool {
        self.done
    }
    pub fn created_at(&self) -> &scalar::Timestamp {
        &self.created_at
    }
}

impl relay::ConnectionNode for Todo {
    fn cursor(&self) -> relay::Cursor {
        relay::Cursor::new(self.id, self.created_at)
    }
    const CONNECTION_TYPE_NAME: &'static str = "TodoConnection";
    const EDGE_TYPE_NAME: &'static str = "TodoEdge";
}
