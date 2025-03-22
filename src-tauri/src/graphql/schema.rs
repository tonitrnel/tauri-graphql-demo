use super::scalar;
use super::context::Context;
use juniper::{graphql_object, EmptySubscription, RootNode};

pub struct Query;

#[graphql_object]  
#[graphql(context = Context, scalar = scalar::CustomScalarValue)]
impl Query{
    pub fn greet(name: String) -> String{
        format!("Hello, {}! You've been greeted from Rust!", name)
    }
}

pub struct Mutation;
#[graphql_object]  
#[graphql(context = Context, scalar = scalar::CustomScalarValue)]  
impl Mutation {  
    pub fn add(a: i32, b: i32) -> i32 {  
        a + b  
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