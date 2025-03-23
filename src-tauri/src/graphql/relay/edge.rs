use juniper::{
    macros::reflect::{BaseSubTypes, BaseType, Type, Types, WrappedType, WrappedValue},
    marker::IsOutputType,
    meta::MetaType,
    Arguments, Context, ExecutionResult, Executor, GraphQLType, GraphQLValue, GraphQLValueAsync,
    Registry, ScalarValue,
};

use super::ConnectionNode;
use super::Cursor;

#[derive(Debug)]
pub struct ConnectionEdge<N> {
    pub(super) node: N,
    pub(super) cursor: Cursor,
}

impl<N, S> GraphQLType<S> for ConnectionEdge<N>
where
    N: GraphQLType<S> + ConnectionNode,
    N::Context: Context,
    S: ScalarValue,
{
    fn name(_info: &Self::TypeInfo) -> Option<&str> {
        Some(N::EDGE_TYPE_NAME)
    }
    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, S>) -> MetaType<'r, S>
    where
        S: 'r,
    {
        let fields = &[
            registry
                .field::<&N>("node", info)
                .description("表示分页结果中的单个数据节点，包含实际业务数据"),
            registry
                .field::<&String>("cursor", &())
                .description("唯一标识分页位置的游标"),
        ];
        registry.build_object_type::<Self>(info, fields).into_meta()
    }
}

impl<N, S> GraphQLValue<S> for ConnectionEdge<N>
where
    N: GraphQLType<S> + ConnectionNode,
    N::Context: Context,
    S: ScalarValue,
{
    type Context = N::Context;
    type TypeInfo = <N as GraphQLValue<S>>::TypeInfo;
    fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
        <Self as GraphQLType<S>>::name(info)
    }
    fn resolve_field(
        &self,
        info: &Self::TypeInfo,
        field_name: &str,
        _arguments: &Arguments<S>,
        executor: &Executor<Self::Context, S>,
    ) -> ExecutionResult<S> {
        match field_name {
            "node" => executor.resolve_with_ctx(info, &self.node),
            "cursor" => executor.resolve_with_ctx(&(), &self.cursor),
            _ => panic!("Field {} not found on type ConnectionEdge", field_name),
        }
    }
    fn concrete_type_name(&self, _context: &Self::Context, info: &Self::TypeInfo) -> String {
        self.type_name(info).unwrap_or("ConnectionEdge").to_string()
    }
}

impl<N, S> GraphQLValueAsync<S> for ConnectionEdge<N>
where
    N: GraphQLType<S> + GraphQLValueAsync<S> + ConnectionNode + Sync + Send,
    N::TypeInfo: Sync,
    N::Context: Context + Sync,
    S: ScalarValue + Send + Sync,
{
    fn resolve_field_async<'a>(
        &'a self,
        info: &'a Self::TypeInfo,
        field_name: &'a str,
        _arguments: &'a Arguments<S>,
        executor: &'a Executor<Self::Context, S>,
    ) -> juniper::BoxFuture<'a, ExecutionResult<S>> {
        let f = async move {
            match field_name {
                "node" => executor.resolve_with_ctx_async(info, &self.node).await,
                "cursor" => executor.resolve_with_ctx(&(), &self.cursor),
                _ => panic!("Field {} not found on type ConnectionEdge", field_name),
            }
        };
        use juniper::futures::future;
        future::FutureExt::boxed(f)
    }
}

impl<N, S> IsOutputType<S> for ConnectionEdge<N>
where
    N: GraphQLType<S> + ConnectionNode,
    S: ScalarValue,
    <N as GraphQLValue<S>>::Context: Context,
{
}

impl<N, S> BaseType<S> for ConnectionEdge<N>
where
    N: GraphQLType<S> + ConnectionNode,
    N::Context: Context,
    S: ScalarValue,
{
    const NAME: Type = N::EDGE_TYPE_NAME;
}

impl<N, S> BaseSubTypes<S> for ConnectionEdge<N>
where
    N: GraphQLType<S> + ConnectionNode + BaseType<S>,
    N::Context: Context,
    S: ScalarValue,
{
    const NAMES: Types = &[<N as BaseType<S>>::NAME, <Cursor as BaseType<S>>::NAME];
}

impl<N, S> WrappedType<S> for ConnectionEdge<N>
where
    N: GraphQLType<S> + ConnectionNode,
    N::Context: Context,
    S: ScalarValue,
{
    const VALUE: WrappedValue = 1;
}
