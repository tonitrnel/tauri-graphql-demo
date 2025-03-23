use super::Cursor;

pub trait ConnectionNode {
    fn cursor(&self) -> Cursor;
    const CONNECTION_TYPE_NAME: &'static str;
    const EDGE_TYPE_NAME: &'static str;
}