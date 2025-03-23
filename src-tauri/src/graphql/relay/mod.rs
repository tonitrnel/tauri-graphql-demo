mod connection;
mod cursor;
mod edge;
mod node;
mod pagination;

pub use connection::*;
pub use cursor::*;
pub use edge::*;
pub use node::*;
pub use pagination::*;

impl<N> Connection<N>
where
    N: ConnectionNode,
{
    fn build_connection(
        pagination: Pagination,
        total_count: i32,
        edges: Vec<N>,
    ) -> juniper::FieldResult<Connection<N>> {
        let edges_len = edges.len() as i32;
        // 前面有 validate 函数约束了 first 和 last 不能同时存在，故此不做额外的判断
        let has_next_page = pagination.first.map(|it| edges_len > it).unwrap_or(false);
        let has_previous_page = pagination.last.map(|it| edges_len > it).unwrap_or(false);
        // 如果 first，last 都没有传，默认取 10 条，但 has_next_page 和 has_previous_page 永远为 false
        let limit = pagination.limit();

        let take_length = i32::min(edges_len, limit);

        let edges = edges
            .into_iter()
            .take(take_length as usize)
            .map(|edge| ConnectionEdge {
                cursor: edge.cursor(),
                node: edge,
            })
            .collect::<Vec<_>>();
        Ok(Self {
            page_info: PageInfo {
                has_previous_page,
                has_next_page,
                start_cursor: edges.first().map(|edge| edge.cursor.clone()),
                end_cursor: edges.last().map(|edge| edge.cursor.clone()),
            },
            edges,
            total_count,
        })
    }

    pub async fn new<'a, C, S, F1, F2>(
        executor: &juniper::Executor<'_, '_, C, S>,
        pagination: Pagination,
        loader: F1,
        total_loader: F2,
    ) -> juniper::FieldResult<Connection<N>>
    where
        S: juniper::ScalarValue + 'a,
        C: 'a,
        F1: AsyncFnOnce(&Pagination) -> anyhow::Result<Vec<N>>,
        F2: AsyncFnOnce() -> anyhow::Result<i32>,
    {
        pagination.validate()?;
        let children: juniper::LookAheadChildren<'_, S> = executor.look_ahead().children();
        let has_total_count_field = children
            .iter()
            .any(|sel| sel.field_original_name() == "totalCount");
        let edges = loader(&pagination).await?;
        let total_count = if has_total_count_field {
            total_loader().await?
        } else {
            0
        };
        Self::build_connection(pagination, total_count, edges)
    }
}
