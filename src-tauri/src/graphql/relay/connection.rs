use super::{ConnectionEdge, ConnectionNode, Cursor};
use juniper::{
    macros::reflect::{BaseSubTypes, BaseType, Type, Types, WrappedType, WrappedValue},
    marker::IsOutputType,
    meta::MetaType,
    Arguments, Context, ExecutionResult, Executor, GraphQLObject, GraphQLType, GraphQLValue,
    GraphQLValueAsync, Registry, ScalarValue,
};

#[derive(Debug, Clone, Default, GraphQLObject)]
pub struct PageInfo {
    /// 是否存在上一页（当使用 last/before 时可用）
    pub(super) has_previous_page: bool,
    /// 是否存在下一页（当使用 first/after 时可用）
    pub(super) has_next_page: bool,
    /// 当前页第一条记录的游标
    pub(super) start_cursor: Option<Cursor>,
    /// 当前页最后一条记录的游标
    pub(super) end_cursor: Option<Cursor>,
}

#[derive(Debug, Default)]
pub struct Connection<N> {
    pub(super) edges: Vec<ConnectionEdge<N>>,
    pub(super) page_info: PageInfo,
    pub(super) total_count: i32,
}

impl<N, S> GraphQLType<S> for Connection<N>
where
    N: GraphQLType<S> + ConnectionNode,
    N::Context: Context,
    S: ScalarValue,
{
    fn name(_info: &<N as GraphQLValue<S>>::TypeInfo) -> Option<&str> {
        Some(N::CONNECTION_TYPE_NAME)
    }
    fn meta<'r>(
        info: &<N as GraphQLValue<S>>::TypeInfo,
        registry: &mut Registry<'r, S>,
    ) -> MetaType<'r, S>
    where
        S: 'r,
    {
        let fields = &[
            registry
                .field::<&Vec<ConnectionEdge<N>>>("edges", info)
                .description("分页连接的核心数据载体，包含节点及其关联的元数据（如游标）"),
            registry
                .field::<&Vec<N>>("nodes", info)
                .description("直接访问节点数据的快捷方式，省略 edges 层"),
            registry
                .field::<&i32>("totalCount", &())
                .description("匹配当前筛选条件的总记录数，不受分页限制"),
            registry
                .field::<&PageInfo>("pageInfo", &())
                .description("分页控制元数据，用于确定是否可翻页及边界游标"),
        ];
        registry.build_object_type::<Self>(info, fields).into_meta()
    }
}

impl<N, S> GraphQLValue<S> for Connection<N>
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
            "edges" => executor.resolve_with_ctx(info, &self.edges),
            "nodes" => {
                let nodes = self.edges.iter().map(|edge| &edge.node).collect::<Vec<_>>();
                executor.resolve_with_ctx(info, &nodes)
            }
            "pageInfo" => executor.resolve_with_ctx(&(), &self.page_info),
            "totalCount" => executor.resolve_with_ctx(&(), &self.total_count),
            _ => panic!("Field {} not found on type Connection", field_name),
        }
    }
    fn concrete_type_name(&self, _context: &Self::Context, info: &Self::TypeInfo) -> String {
        self.type_name(info).unwrap_or("Connection").to_string()
    }
}

impl<N, S> GraphQLValueAsync<S> for Connection<N>
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
                "edges" => executor.resolve_with_ctx_async(info, &self.edges).await,
                "nodes" => {
                    let nodes = self.edges.iter().map(|edge| &edge.node).collect::<Vec<_>>();
                    executor.resolve_with_ctx_async(info, &nodes).await
                }
                "pageInfo" => executor.resolve_with_ctx(&(), &self.page_info),
                "totalCount" => executor.resolve_with_ctx(&(), &self.total_count),
                _ => panic!("Field {} not found on type Connection", field_name),
            }
        };
        use juniper::futures::future;
        future::FutureExt::boxed(f)
    }
}

impl<N, S> IsOutputType<S> for Connection<N>
where
    N: GraphQLType<S> + ConnectionNode,
    S: ScalarValue,
    <N as GraphQLValue<S>>::Context: Context,
{
}

impl<N, S> BaseType<S> for Connection<N>
where
    N: GraphQLType<S> + ConnectionNode,
    S: ScalarValue,
{
    const NAME: Type = N::CONNECTION_TYPE_NAME;
}

impl<N, S> BaseSubTypes<S> for Connection<N>
where
    N: GraphQLType<S> + ConnectionNode + BaseType<S>,
    N::Context: Context,
    S: ScalarValue,
{
    const NAMES: Types = &[
        <ConnectionEdge<N> as BaseType<S>>::NAME,
        <PageInfo as BaseType<S>>::NAME,
    ];
}

impl<N, S> WrappedType<S> for Connection<N>
where
    N: GraphQLType<S> + ConnectionNode,
    S: ScalarValue,
{
    const VALUE: WrappedValue = 1;
}
